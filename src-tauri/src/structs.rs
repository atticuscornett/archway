use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct InputFile {
    pub path_type: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct FilterTraits {
    pub size: Option<u64>,
    pub period: Option<String>,
    pub extensions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct FileFilter {
    pub filter_type: String,
    pub traits: FilterTraits,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct TriggerTraits {
    pub time: Option<Vec<String>>,
    pub event: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Trigger {
    pub trigger_type: String,
    pub traits: TriggerTraits,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct JobInfo {
    pub job_name: String,
    pub uuid: String,
    pub file_behavior: String,
    pub input_dirs: Vec<InputFile>,
    pub output_dir: String,
    pub output_device: String,
    pub copies: u32,
    pub portable: bool,
    pub new_folder: bool,
    pub file_filters: Vec<FileFilter>,
    pub triggers: Vec<Trigger>,
    pub version: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct DriveInfoFile {
    pub uuid: String,
    pub jobs: Vec<JobInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct JobStatus {
    pub job: JobInfo,
    pub step: u32,
    pub total_steps: u32,
    pub step_title: String,
    pub last_action: String,
    pub success: bool,
    pub completed: bool,
    pub percent: f32,
}
