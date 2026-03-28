use serde::Deserialize;

/// A parsed tool call from the LLM.
#[derive(Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

// ── Per-tool argument structs ───────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ReadFileArgs {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct WriteFileArgs {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ShellArgs {
    pub command: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchArgs {
    pub pattern: String,
    pub path: Option<String>,
}

use crate::sandbox::Sandbox;

/// Dispatch a tool call through the sandbox and return the result string.
pub fn execute(sandbox: &Sandbox, call: &ToolCall) -> String {
    match call.name.as_str() {
        "read_file" => match serde_json::from_str::<ReadFileArgs>(&call.arguments) {
            Ok(args) => match sandbox.read_file(&args.path) {
                Ok(content) => content,
                Err(e) => format!("error: {e}"),
            },
            Err(e) => format!("invalid arguments: {e}"),
        },

        "write_file" => match serde_json::from_str::<WriteFileArgs>(&call.arguments) {
            Ok(args) => match sandbox.write_file(&args.path, &args.content) {
                Ok(()) => format!("wrote {} bytes to {}", args.content.len(), args.path),
                Err(e) => format!("error: {e}"),
            },
            Err(e) => format!("invalid arguments: {e}"),
        },

        "shell" => match serde_json::from_str::<ShellArgs>(&call.arguments) {
            Ok(args) => match sandbox.exec(&args.command) {
                Ok(output) => output,
                Err(e) => format!("error: {e}"),
            },
            Err(e) => format!("invalid arguments: {e}"),
        },

        "search" => match serde_json::from_str::<SearchArgs>(&call.arguments) {
            Ok(args) => match sandbox.search(&args.pattern, args.path.as_deref()) {
                Ok(results) => results,
                Err(e) => format!("error: {e}"),
            },
            Err(e) => format!("invalid arguments: {e}"),
        },

        other => format!("unknown tool: {other}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SandboxConfig;
    use std::fs;

    fn test_sandbox() -> (Sandbox, tempfile::TempDir) {
        let tmp = tempfile::tempdir().unwrap();
        let config = SandboxConfig {
            root: tmp.path().to_path_buf(),
            allowed_commands: vec!["echo".into(), "ls".into()],
            require_approval: false,
        };
        (Sandbox::new(&config), tmp)
    }

    #[test]
    fn execute_read_file() {
        let (sb, tmp) = test_sandbox();
        fs::write(tmp.path().join("hello.txt"), "world").unwrap();
        let call = ToolCall {
            id: "1".into(),
            name: "read_file".into(),
            arguments: r#"{"path": "hello.txt"}"#.into(),
        };
        let result = execute(&sb, &call);
        assert_eq!(result, "world");
    }

    #[test]
    fn execute_write_file() {
        let (sb, tmp) = test_sandbox();
        let call = ToolCall {
            id: "2".into(),
            name: "write_file".into(),
            arguments: r#"{"path": "out.txt", "content": "data"}"#.into(),
        };
        let result = execute(&sb, &call);
        assert!(result.contains("wrote 4 bytes"));
        let content = fs::read_to_string(tmp.path().join("out.txt")).unwrap();
        assert_eq!(content, "data");
    }

    #[test]
    fn execute_shell() {
        let (sb, _tmp) = test_sandbox();
        let call = ToolCall {
            id: "3".into(),
            name: "shell".into(),
            arguments: r#"{"command": "echo hi"}"#.into(),
        };
        let result = execute(&sb, &call);
        assert!(result.contains("hi"));
    }

    #[test]
    fn execute_shell_blocked() {
        let (sb, _tmp) = test_sandbox();
        let call = ToolCall {
            id: "4".into(),
            name: "shell".into(),
            arguments: r#"{"command": "rm -rf /"}"#.into(),
        };
        let result = execute(&sb, &call);
        assert!(result.contains("error: command not allowed"));
    }

    #[test]
    fn execute_unknown_tool() {
        let (sb, _tmp) = test_sandbox();
        let call = ToolCall {
            id: "5".into(),
            name: "nonexistent".into(),
            arguments: "{}".into(),
        };
        let result = execute(&sb, &call);
        assert!(result.contains("unknown tool"));
    }

    #[test]
    fn execute_bad_arguments() {
        let (sb, _tmp) = test_sandbox();
        let call = ToolCall {
            id: "6".into(),
            name: "read_file".into(),
            arguments: "not json".into(),
        };
        let result = execute(&sb, &call);
        assert!(result.contains("invalid arguments"));
    }
}
