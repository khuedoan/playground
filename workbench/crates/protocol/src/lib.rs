use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesiredState {
    Running,
    Stopped,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActualState {
    Pending,
    Starting,
    Running,
    Stopping,
    Stopped,
    Deleted,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct VmProfile {
    pub vcpus: u16,
    pub memory_mib: u32,
    pub disk_gib: u32,
    #[serde(default = "default_true")]
    pub gui: bool,
}

impl Default for VmProfile {
    fn default() -> Self {
        Self {
            vcpus: 4,
            memory_mib: 8192,
            disk_gib: 40,
            gui: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsureVmRequest {
    pub command_id: Uuid,
    pub workspace_id: Uuid,
    pub generation: u64,
    pub desired_state: DesiredState,
    #[serde(default)]
    pub profile: VmProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmStatus {
    pub workspace_id: Uuid,
    pub generation: u64,
    pub command_id: Uuid,
    pub desired_state: DesiredState,
    pub actual_state: ActualState,
    pub ip_address: Option<String>,
    pub desktop_url: Option<String>,
    pub code_url: Option<String>,
    pub agent_url: Option<String>,
    pub error: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecRequest {
    pub command: String,
    #[serde(default = "default_cwd")]
    pub cwd: String,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecResponse {
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiRpcRequest {
    pub request: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiRpcResponse {
    pub response: Value,
    pub preceding_events: Vec<Value>,
}

fn default_true() -> bool {
    true
}

fn default_cwd() -> String {
    "/workspace".to_owned()
}

fn default_timeout() -> u64 {
    120
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_defaults_are_desktop_ready() {
        let value: VmProfile = serde_json::from_str("{}").unwrap();
        assert_eq!(value.vcpus, 4);
        assert_eq!(value.memory_mib, 8192);
        assert!(value.gui);
    }
}
