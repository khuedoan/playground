use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::SandboxConfig;

/// Security sandbox that restricts file and command access.
#[derive(Debug, Clone)]
pub struct Sandbox {
    /// Canonical root directory — all file operations are confined here.
    root: PathBuf,
    /// Commands that may be run without interactive approval.
    allowed_commands: Vec<String>,
    /// Whether destructive writes require user confirmation.
    require_approval: bool,
}

impl Sandbox {
    pub fn new(config: &SandboxConfig) -> Self {
        let root = config
            .root
            .canonicalize()
            .unwrap_or_else(|_| config.root.clone());
        Self {
            root,
            allowed_commands: config.allowed_commands.clone(),
            require_approval: config.require_approval,
        }
    }

    // ── Path validation ─────────────────────────────────────────────

    /// Resolve a (possibly relative) path and verify it lives under `root`.
    pub fn resolve_path(&self, path: &str) -> Result<PathBuf, String> {
        let candidate = if Path::new(path).is_absolute() {
            PathBuf::from(path)
        } else {
            self.root.join(path)
        };

        // Canonicalize what exists; for new files, canonicalize the parent.
        let resolved = if candidate.exists() {
            candidate
                .canonicalize()
                .map_err(|e| format!("cannot resolve path: {e}"))?
        } else {
            let parent = candidate
                .parent()
                .ok_or_else(|| "invalid path: no parent directory".to_string())?;
            let parent_canon = parent
                .canonicalize()
                .map_err(|e| format!("cannot resolve parent: {e}"))?;
            parent_canon.join(candidate.file_name().unwrap_or_default())
        };

        if !resolved.starts_with(&self.root) {
            return Err(format!(
                "access denied: {} is outside sandbox root {}",
                resolved.display(),
                self.root.display()
            ));
        }

        Ok(resolved)
    }

    // ── File operations ─────────────────────────────────────────────

    pub fn read_file(&self, path: &str) -> Result<String, String> {
        let resolved = self.resolve_path(path)?;
        std::fs::read_to_string(&resolved).map_err(|e| format!("read error: {e}"))
    }

    pub fn write_file(&self, path: &str, content: &str) -> Result<(), String> {
        // For writes, we do a looser path check: ensure the path doesn't escape
        // the root via ".." traversal, then create parent dirs as needed.
        let candidate = if Path::new(path).is_absolute() {
            PathBuf::from(path)
        } else {
            self.root.join(path)
        };

        // Walk the components to detect traversal without requiring existing dirs.
        let mut resolved = self.root.clone();
        let relative = candidate
            .strip_prefix(&self.root)
            .map_err(|_| {
                format!(
                    "access denied: {} is outside sandbox root {}",
                    candidate.display(),
                    self.root.display()
                )
            })?;
        for component in relative.components() {
            match component {
                std::path::Component::ParentDir => {
                    if !resolved.starts_with(&self.root) || resolved == self.root {
                        return Err(format!(
                            "access denied: path escapes sandbox root {}",
                            self.root.display()
                        ));
                    }
                    resolved.pop();
                }
                std::path::Component::Normal(seg) => resolved.push(seg),
                _ => {}
            }
        }
        if !resolved.starts_with(&self.root) {
            return Err(format!(
                "access denied: {} is outside sandbox root {}",
                resolved.display(),
                self.root.display()
            ));
        }

        if self.require_approval {
            eprint!("write to {} — approve? [y/N] ", resolved.display());
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .map_err(|e| format!("stdin error: {e}"))?;
            if !input.trim().eq_ignore_ascii_case("y") {
                return Err("write rejected by user".into());
            }
        }

        if let Some(parent) = resolved.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("mkdir error: {e}"))?;
        }
        std::fs::write(&resolved, content).map_err(|e| format!("write error: {e}"))
    }

    // ── Command execution ───────────────────────────────────────────

    /// Check whether a command is on the allowlist.
    fn is_command_allowed(&self, cmd: &str) -> bool {
        // Extract the base command (first word, ignore arguments).
        let base = cmd.split_whitespace().next().unwrap_or("");
        // Also strip any path prefix to get just the binary name.
        let binary = Path::new(base)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or(base);
        self.allowed_commands.iter().any(|a| a == binary)
    }

    pub fn exec(&self, cmd: &str) -> Result<String, String> {
        if !self.is_command_allowed(cmd) {
            return Err(format!(
                "command not allowed: {cmd}\nallowed: {:?}",
                self.allowed_commands
            ));
        }

        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(&self.root)
            .output()
            .map_err(|e| format!("exec error: {e}"))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let mut result = String::new();

        if !stdout.is_empty() {
            result.push_str(&stdout);
        }
        if !stderr.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str("STDERR:\n");
            result.push_str(&stderr);
        }
        if !output.status.success() {
            result.push_str(&format!("\n(exit code: {})", output.status));
        }

        Ok(result)
    }

    // ── Search ──────────────────────────────────────────────────────

    pub fn search(&self, pattern: &str, path: Option<&str>) -> Result<String, String> {
        let search_path = if let Some(p) = path {
            self.resolve_path(p)?
        } else {
            self.root.clone()
        };

        let output = Command::new("grep")
            .args(["-rn", "--color=never", pattern])
            .arg(&search_path)
            .output()
            .map_err(|e| format!("search error: {e}"))?;

        let result = String::from_utf8_lossy(&output.stdout);
        if result.is_empty() {
            Ok("no matches found".into())
        } else {
            // Truncate very large results
            let truncated: String = result.chars().take(10_000).collect();
            if truncated.len() < result.len() {
                Ok(format!("{truncated}\n... (truncated)"))
            } else {
                Ok(truncated)
            }
        }
    }

    /// Returns the sandbox root path.
    pub fn root(&self) -> &Path {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn test_sandbox() -> (Sandbox, tempfile::TempDir) {
        let tmp = tempfile::tempdir().unwrap();
        let config = SandboxConfig {
            root: tmp.path().to_path_buf(),
            allowed_commands: vec![
                "ls".into(),
                "echo".into(),
                "cat".into(),
                "grep".into(),
            ],
            require_approval: false,
        };
        (Sandbox::new(&config), tmp)
    }

    #[test]
    fn resolve_path_inside_root() {
        let (sb, _tmp) = test_sandbox();
        let resolved = sb.resolve_path("foo.txt");
        assert!(resolved.is_ok());
    }

    #[test]
    fn resolve_path_rejects_traversal() {
        let (sb, _tmp) = test_sandbox();
        let result = sb.resolve_path("../../etc/passwd");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("outside sandbox root"));
    }

    #[test]
    fn read_and_write_file() {
        let (sb, tmp) = test_sandbox();
        let file_path = tmp.path().join("test.txt");
        fs::write(&file_path, "hello world").unwrap();
        let content = sb.read_file("test.txt").unwrap();
        assert_eq!(content, "hello world");
    }

    #[test]
    fn write_creates_parent_dirs() {
        let (sb, _tmp) = test_sandbox();
        sb.write_file("sub/dir/file.txt", "nested").unwrap();
        let content = sb.read_file("sub/dir/file.txt").unwrap();
        assert_eq!(content, "nested");
    }

    #[test]
    fn exec_allowed_command() {
        let (sb, _tmp) = test_sandbox();
        let result = sb.exec("echo hello").unwrap();
        assert!(result.contains("hello"));
    }

    #[test]
    fn exec_rejects_disallowed_command() {
        let (sb, _tmp) = test_sandbox();
        let result = sb.exec("rm -rf /");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("command not allowed"));
    }

    #[test]
    fn command_allowlist_checks_binary_name() {
        let (sb, _tmp) = test_sandbox();
        assert!(sb.is_command_allowed("ls -la"));
        assert!(sb.is_command_allowed("echo foo bar"));
        assert!(!sb.is_command_allowed("curl http://example.com"));
        assert!(!sb.is_command_allowed("rm file.txt"));
    }

    #[test]
    fn search_finds_pattern() {
        let (sb, tmp) = test_sandbox();
        fs::write(tmp.path().join("haystack.txt"), "needle in a haystack\nno match here").unwrap();
        let result = sb.search("needle", None).unwrap();
        assert!(result.contains("needle"));
    }

    #[test]
    fn search_returns_no_matches() {
        let (sb, tmp) = test_sandbox();
        fs::write(tmp.path().join("empty.txt"), "nothing here").unwrap();
        let result = sb.search("zzzzzzz", None).unwrap();
        assert_eq!(result, "no matches found");
    }
}
