use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct InputFile {
    pub path_type: String,
    pub path: String
}

#[derive(Serialize, Deserialize)]
pub(crate) struct FilterTraits {
    pub size : Option<u64>,
    pub lastused: Option<String>,
    pub extensions: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
pub(crate) struct FileFilter {
    pub filter_type: String,
    pub traits: FilterTraits
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TriggerTraits {
    pub time: Option<Vec<String>>,
    pub event: Option<String>
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Trigger {
    pub trigger_type: String,
    pub traits: TriggerTraits
}

#[derive(Serialize, Deserialize)]
pub(crate) struct JobInfo {
    pub job_name: String,
    pub uuid: String,
    pub file_behavior: String,
    pub input_dirs: Vec<InputFile>,
    pub output_dir: String,
    pub output_device: String,
    pub copies: u32,
    pub portable: bool,
    pub file_filters: Vec<FileFilter>,
    pub triggers: Vec<Trigger>,
    pub version: u32
}