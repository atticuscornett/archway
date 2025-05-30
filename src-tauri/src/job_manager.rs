use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::storage_manager;
use crate::structs::{JobInfo, JobStatus};

static JOB_STATUSES: Lazy<Mutex<Vec<JobStatus>>> = Lazy::new(|| Mutex::new(Vec::new()));

/*
Job Steps:
1. Indexing files to move
2. Copying files
3. Verifying files
4. (If moving) Deleting original files
 */

pub fn start_job(uuid: String) -> bool {
    let already_running = {
        let job_statuses = JOB_STATUSES.lock().unwrap();
        job_statuses.iter().any(|js| js.job.uuid == uuid)
    };
    if already_running {
        println!("Job with UUID {} is already running.", uuid);
        return false;
    }


    let new_job = storage_manager::get_job_by_uuid(&uuid);

    let new_job_status = JobStatus {
        job: new_job.clone(),
        step: 0,
        total_steps: if new_job.clone().file_behavior == "copy" { 3 } else { 4 }, // If copying, skip the deletion step
        step_title: String::from("Initializing Job"),
        last_action: String::from("Starting job..."),
        success: true,
        completed: false,
        percent: 0.0
    };

    JOB_STATUSES.lock().unwrap().push(new_job_status);

    tauri::async_runtime::spawn(job_stage_one(uuid));

    true

}

pub fn get_all_job_statuses() -> Vec<JobStatus> {
    JOB_STATUSES.lock().unwrap().clone()
}

fn update_job_status(uuid: &str, step: u32, step_title: String, last_action: String, success: bool, completed: bool, percent: f32) {
    let mut job_statuses = JOB_STATUSES.lock().unwrap();
    if let Some(job_status) = job_statuses.iter_mut().find(|js| js.job.uuid == uuid) {
        job_status.step = step;
        job_status.step_title = step_title;
        job_status.last_action = last_action;
        job_status.success = success;
        job_status.completed = completed;
        job_status.percent = percent;
    }
}

fn update_last_action(uuid: &str, last_action: String) {
    let mut job_statuses = JOB_STATUSES.lock().unwrap();
    if let Some(job_status) = job_statuses.iter_mut().find(|js| js.job.uuid == uuid) {
        job_status.last_action = last_action;
    }
}

fn get_all_subfolders(path: &str) -> Vec<String> {
    let mut subfolders = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                let subfolder_path = entry.path().to_string_lossy().to_string();
                subfolders.push(subfolder_path.clone());
                subfolders.extend(get_all_subfolders(&subfolder_path));
            }
        }
    }
    subfolders
}

fn get_all_files(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                files.push(entry.path().to_string_lossy().to_string());
            } else if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                files.extend(get_all_files(&entry.path().to_string_lossy()));
            }
        }
    }
    files
}

async fn job_stage_one(uuid: String) {
    update_job_status(uuid.as_str(), 1, String::from("Indexing Files"),
                      String::from("Getting all input folders..."), true, false, -1.0);

    let input_dirs = storage_manager::get_job_by_uuid(&uuid).input_dirs;
    let mut all_folders: Vec<String> = Vec::new();
    for input_dir in input_dirs {
        if (input_dir.path_type == "library") {
            if (input_dir.path == "documents") {
                let documents_path = dirs::document_dir().unwrap();
                all_folders.push(documents_path.to_string_lossy().to_string());
            } else if (input_dir.path == "downloads") {
                let downloads_path = dirs::download_dir().unwrap();
                all_folders.push(downloads_path.to_string_lossy().to_string());
            } else if (input_dir.path == "desktop") {
                let desktop_path = dirs::desktop_dir().unwrap();
                all_folders.push(desktop_path.to_string_lossy().to_string());
            } else if (input_dir.path == "music") {
                let music_path = dirs::audio_dir().unwrap();
                all_folders.push(music_path.to_string_lossy().to_string());
            } else if (input_dir.path == "pictures") {
                let pictures_path = dirs::picture_dir().unwrap();
                all_folders.push(pictures_path.to_string_lossy().to_string());
            } else if (input_dir.path == "videos") {
                let videos_path = dirs::video_dir().unwrap();
                all_folders.push(videos_path.to_string_lossy().to_string());
            } else {
                println!("Unknown library path: {}", input_dir.path);
            }
        }
        all_folders.push(input_dir.path.clone());
        all_folders.extend(get_all_subfolders(&input_dir.path));
    }

    update_last_action(uuid.as_str(), String::from("Getting all files..."));
    let mut all_files: Vec<String> = Vec::new();
    for input_dir in all_folders.iter() {
        all_files.extend(get_all_files(input_dir.as_str()));
    }

    update_last_action(uuid.as_str(), String::from("Applying filters..."));
    let filters = storage_manager::get_job_by_uuid(&uuid).file_filters;
    for filter in filters {
        if filter.filter_type == "extension" {
            let mut allowed_extensions = filter.traits.extensions.unwrap();
            for extension in allowed_extensions.clone() {
                if (extension == "documents:special"){
                    allowed_extensions.extend(vec![String::from("doc"), String::from("docx"),
                    String::from("pdf"), String::from("txt"), String::from("odt"), String::from("rtf"),
                        String::from("md"), String::from("epub"), String::from("pptx"), String::from("xls"), String::from("xlsx")]);

                    println!("Allowed extensions: {:?}", allowed_extensions);
                }
            }

            all_files.retain(|file| {
                let file_extension = file.split('.').last().unwrap_or("");
                allowed_extensions.contains(&file_extension.to_lowercase())
            });
        }
    }


    println!("All folders to move: {:?}", all_folders);
    println!("All files to move: {:?}", all_files);



}
