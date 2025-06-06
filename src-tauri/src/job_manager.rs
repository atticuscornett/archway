use std::collections::HashMap;
use crate::structs::{JobInfo, JobStatus};
use crate::{drive_manager, storage_manager};
use once_cell::sync::{Lazy, OnceCell};
use sha2::{Digest, Sha256};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;
use crate::storage_manager::set_job_health_by_uuid;

static APP_HANDLE: OnceCell<Mutex<AppHandle>> = OnceCell::new();
static JOB_STATUSES: Lazy<Mutex<Vec<JobStatus>>> = Lazy::new(|| Mutex::new(Vec::new()));
static JOB_UPDATES: OnceCell<Mutex<HashMap<String, String>>> = OnceCell::new();

/*
Job Steps:
1. Indexing files to move
2. Initialize directories
3. Copying files
4. Verifying files
5. (If moving) Deleting original files
 */

pub fn set_app_handle(app_handle: AppHandle) {
    APP_HANDLE.set(Mutex::new(app_handle)).unwrap();
}

pub fn set_job_update(uuid: String, update: String) {
    let mut job_updates = JOB_UPDATES
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("Failed to lock job updates");
    job_updates.insert(uuid, update);
}

pub fn get_job_update(uuid: String) -> String {
    let job_updates = JOB_UPDATES
        .get()
        .expect("Job updates not initialized")
        .lock()
        .expect("Failed to lock job updates");
    if (job_updates.contains_key(&uuid)) {
        job_updates.get(&uuid).unwrap().clone()
    } else {
        "not_running".to_string()
    }
}

pub fn get_app_handle() -> AppHandle {
    APP_HANDLE
        .get()
        .expect("App handle not set")
        .lock()
        .expect("Failed to lock app handle")
        .clone()
}

pub fn start_job(uuid: String) -> bool {
    // Check if the job is already running
    let already_running = {
        let job_statuses = JOB_STATUSES.lock().unwrap();
        job_statuses
            .iter()
            .any(|js| js.job.uuid == uuid && !js.completed)
    };

    // If the job is already running, do not start it again
    if already_running {
        println!("Job with UUID {} is already running.", uuid);
        return false;
    }

    clear_job(&uuid);

    let new_job = storage_manager::get_job_by_uuid(&uuid);

    let new_job_status = JobStatus {
        job: new_job.clone(),
        step: 0,
        total_steps: if new_job.clone().file_behavior == "copy" {
            4
        } else {
            5
        }, // If copying, skip the deletion step
        step_title: String::from("Initializing Job"),
        last_action: String::from("Starting job..."),
        success: true,
        completed: false,
        percent: 0.0,
    };

    JOB_STATUSES.lock().unwrap().push(new_job_status);
    set_job_update(uuid.clone(), "running".to_string());

    tauri::async_runtime::spawn(job_stage_one(uuid));

    get_app_handle()
        .notification()
        .builder()
        .title("Job Started: ".to_owned() + &new_job.job_name)
        .body("The job has been started successfully.")
        .show()
        .unwrap();

    // Return true to indicate the job has started successfully
    true
}

pub fn clear_job(uuid: &str) -> bool {
    let mut job_statuses = JOB_STATUSES.lock().unwrap();
    if let Some(pos) = job_statuses.iter().position(|js| js.job.uuid == uuid) {
        job_statuses.remove(pos);
        return true;
    }
    false
}

pub fn get_all_job_statuses() -> Vec<JobStatus> {
    JOB_STATUSES.lock().unwrap().clone()
}

fn update_job_status(
    uuid: &str,
    step: u32,
    step_title: String,
    last_action: String,
    success: bool,
    completed: bool,
    percent: f32,
) {
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

pub fn clear_completed_jobs() {
    let mut job_statuses = JOB_STATUSES.lock().unwrap();
    job_statuses.retain(|js| !js.completed);
}

fn update_job_progress(uuid: &str, percent: f32) {
    let mut job_statuses = JOB_STATUSES.lock().unwrap();
    if let Some(job_status) = job_statuses.iter_mut().find(|js| js.job.uuid == uuid) {
        job_status.percent = percent;
    }
}

// Gets all subfolders recursively from a given path
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

// Gets all files recursively from a given path
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

// Gets the last access time of a file in seconds since UNIX_EPOCH
fn get_last_access_time(path: &str) -> std::io::Result<u64> {
    let metadata = fs::metadata(path)?;
    let atime = metadata.accessed()?;
    let duration = atime.duration_since(UNIX_EPOCH).unwrap();
    Ok(duration.as_secs())
}

// Checks if a file's last access time is older than a specified period
fn check_older_than(time: u64, period: &str) -> bool {
    let now = SystemTime::now();
    let period_duration = match period {
        "week" => 7 * 24 * 60 * 60,      // 1 week in seconds
        "2weeks" => 14 * 24 * 60 * 60,   // 2 weeks in seconds
        "month" => 30 * 24 * 60 * 60,    // 1 month in seconds
        "2months" => 60 * 24 * 60 * 60,  // 2 months in seconds
        "3months" => 90 * 24 * 60 * 60,  // 3 months in seconds
        "6months" => 180 * 24 * 60 * 60, // 6 months in seconds
        "year" => 365 * 24 * 60 * 60,    // 1 year in seconds
        _ => 999999999999999,            // Invalid period
    };

    if let Ok(duration) = now.duration_since(UNIX_EPOCH) {
        if duration.as_secs() > time + period_duration {
            true
        } else {
            false
        }
    } else {
        false
    }
}

// Gets the size of a file in bytes
fn get_file_size(path: &str) -> std::io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

fn file_hash(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = fs::File::open(&path)?;
    let mut hasher = Sha256::new();
    let n = io::copy(&mut file, &mut hasher)?;
    Ok(hasher.finalize().to_vec())
}

fn compare_files(file1: &str, file2: &str) -> std::io::Result<bool> {
    let hash1 = file_hash(file1)?;
    let hash2 = file_hash(file2)?;
    Ok(hash1 == hash2)
}

// Stage one of the job: Indexing files to move
async fn job_stage_one(uuid: String) {
    update_job_status(
        uuid.as_str(),
        1,
        String::from("Indexing Files"),
        String::from("Getting all input folders..."),
        true,
        false,
        -1.0,
    );

    let input_dirs = storage_manager::get_job_by_uuid(&uuid).input_dirs;
    let mut all_folders: Vec<String> = Vec::new();

    // Convert library paths to actual directories
    for input_dir in input_dirs {
        if input_dir.path_type == "library" {
            if input_dir.path == "documents" {
                let documents_path = dirs::document_dir().unwrap();
                all_folders.push(documents_path.to_string_lossy().to_string());
            } else if input_dir.path == "downloads" {
                let downloads_path = dirs::download_dir().unwrap();
                all_folders.push(downloads_path.to_string_lossy().to_string());
            } else if input_dir.path == "desktop" {
                let desktop_path = dirs::desktop_dir().unwrap();
                all_folders.push(desktop_path.to_string_lossy().to_string());
            } else if input_dir.path == "music" {
                let music_path = dirs::audio_dir().unwrap();
                all_folders.push(music_path.to_string_lossy().to_string());
            } else if input_dir.path == "pictures" {
                let pictures_path = dirs::picture_dir().unwrap();
                all_folders.push(pictures_path.to_string_lossy().to_string());
            } else if input_dir.path == "videos" {
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
    update_job_progress(uuid.as_str(), 0.33);
    let mut all_files: Vec<String> = Vec::new();
    // Get all files from the input directories
    for input_dir in all_folders.iter() {
        all_files.extend(get_all_files(input_dir.as_str()));
    }

    update_last_action(uuid.as_str(), String::from("Applying filters..."));
    update_job_progress(uuid.as_str(), 0.66);
    let filters = storage_manager::get_job_by_uuid(&uuid).file_filters;
    // Apply filters to the files
    for filter in filters {
        // Apply extension filter
        if filter.filter_type == "extension" {
            let mut allowed_extensions = filter.traits.extensions.unwrap();
            for extension in allowed_extensions.clone() {
                if extension == "documents:special" {
                    allowed_extensions.extend(vec![
                        String::from("doc"),
                        String::from("docx"),
                        String::from("pdf"),
                        String::from("txt"),
                        String::from("odt"),
                        String::from("rtf"),
                        String::from("md"),
                        String::from("epub"),
                        String::from("pptx"),
                        String::from("xls"),
                        String::from("xlsx"),
                    ]);
                }
                if extension == "videos:special" {
                    allowed_extensions.extend(vec![
                        String::from("mp4"),
                        String::from("mkv"),
                        String::from("avi"),
                        String::from("mov"),
                        String::from("wmv"),
                        String::from("flv"),
                        String::from("webm"),
                        String::from("mpeg"),
                    ]);
                }
                if extension == "pictures:special" {
                    allowed_extensions.extend(vec![
                        String::from("jpg"),
                        String::from("jpeg"),
                        String::from("png"),
                        String::from("gif"),
                        String::from("bmp"),
                        String::from("tiff"),
                        String::from("webp"),
                        String::from("svg"),
                    ]);
                }
                if extension == "music:special" {
                    allowed_extensions.extend(vec![
                        String::from("mp3"),
                        String::from("wav"),
                        String::from("flac"),
                        String::from("aac"),
                        String::from("ogg"),
                        String::from("m4a"),
                        String::from("wma"),
                    ]);
                }
                if extension == "archives:special" {
                    allowed_extensions.extend(vec![
                        String::from("zip"),
                        String::from("rar"),
                        String::from("tar"),
                        String::from("gz"),
                        String::from("7z"),
                        String::from("bz2"),
                        String::from("xz"),
                    ]);
                }
            }

            all_files.retain(|file| {
                let file_extension = file.split('.').last().unwrap_or("");
                allowed_extensions.contains(&file_extension.to_lowercase())
            });
        }
        // Apply size filter
        if filter.filter_type == "size" {
            let threshold = filter.traits.size.unwrap();

            all_files.retain(|file| {
                match get_file_size(file.as_str()) {
                    Ok(size) => (size / 1000) / 1000 >= threshold,
                    Err(_) => {
                        println!("Could not get size for file: {}", file);
                        false
                    } // If we can't get the size, exclude the file
                }
            });
        }
        // Apply last used filter
        if filter.filter_type == "last-used" {
            let threshold = filter.traits.period.unwrap();
            all_files.retain(|file| {
                match get_last_access_time(file.as_str()) {
                    Ok(last_accessed) => check_older_than(last_accessed, threshold.as_str()),
                    Err(_) => {
                        println!("Could not get last accessed time for file: {}", file);
                        false
                    } // If we can't get the last accessed time, exclude the file
                }
            });
        }
        if handle_pause_stop(uuid.clone()){
            return;
        }
    }

    println!("All folders to move: {:?}", all_folders);
    println!("All files to move: {:?}", all_files);

    tauri::async_runtime::spawn(job_stage_two(uuid, all_files));
}

// Stage two of the job: Initializing directories
async fn job_stage_two(uuid: String, files: Vec<String>) {
    update_job_status(
        uuid.as_str(),
        2,
        String::from("Initializing Directories"),
        String::from("Initializing directories..."),
        true,
        false,
        -1.0,
    );

    let job_info = storage_manager::get_job_by_uuid(&uuid);

    let job_type = job_info.file_behavior.clone();
    let copies = job_info.copies.clone();

    let output_device = job_info.output_device.clone();

    let mut output_dir = job_info.output_dir.clone();
    let drive = drive_manager::get_root_drive(output_dir.as_str()).unwrap();
    let drive_uuid = drive_manager::get_drive_uuid(drive.as_str());

    if !std::path::Path::new(&drive).exists() {
        println!("Drive does not exist: {}", drive);
        update_job_status(
            uuid.as_str(),
            2,
            String::from("Job failed."),
            String::from("Drive does not exist."),
            false,
            true,
            0.0,
        );
        set_job_update(uuid.clone(), "not_running".to_string());
        job_failed_notification(job_info.uuid);
        return;
    }

    // Ensure the output device matches the drive UUID
    if output_device != "special:any" {
        if drive_uuid.is_empty() {
            println!("Failed to get or create drive UUID.");
            update_job_status(
                uuid.as_str(),
                2,
                String::from("Job failed."),
                String::from("Failed to get or create drive UUID."),
                false,
                true,
                0.0,
            );
            return;
        }
        if drive_uuid != output_device {
            println!(
                "Drive UUID does not match job output device: {} != {}",
                drive_uuid, output_device
            );
            update_job_status(
                uuid.as_str(),
                2,
                String::from("Job failed."),
                String::from("Drive UUID does not match job output device."),
                false,
                true,
                0.0,
            );
            set_job_update(uuid.clone(), "not_running".to_string());
            job_failed_notification(job_info.uuid);
            return;
        }
    }

    // Ensure the output directory exists
    if !std::path::Path::new(&output_dir).exists() {
        if job_info.new_folder.clone() {
            match std::fs::create_dir_all(&output_dir) {
                Ok(_) => println!("Created output directory: {}", output_dir),
                Err(e) => {
                    println!("Failed to create output directory: {}", e);
                    update_job_status(
                        uuid.as_str(),
                        2,
                        String::from("Job failed."),
                        String::from("Failed to create output directory."),
                        false,
                        true,
                        0.0,
                    );
                    set_job_update(uuid.clone(), "not_running".to_string());
                    job_failed_notification(job_info.uuid);
                    return;
                }
            }
        } else {
            println!(
                "Output directory does not exist and new_folder is false: {}",
                output_dir
            );
            update_job_status(
                uuid.as_str(),
                2,
                String::from("Job failed."),
                String::from("Output directory does not exist."),
                false,
                true,
                0.0,
            );
            set_job_update(uuid.clone(), "not_running".to_string());
            job_failed_notification(job_info.uuid);

            return;
        }
    }

    // Create job directory
    let mut output_dir_path = std::path::PathBuf::from(&output_dir);
    output_dir_path = output_dir_path.join(format!("archway-{}", job_info.uuid));
    output_dir = output_dir_path.to_string_lossy().to_string();

    // Check if the output directory already exists and handle copies
    if job_type == "copy" && copies > 1 {
        let mut folder_num = 1;
        output_dir = output_dir_path
            .with_file_name(format!("archway-{}-{}", job_info.uuid, copies))
            .to_string_lossy()
            .to_string();
        while output_dir_path.exists() && folder_num <= copies {
            output_dir = output_dir_path
                .with_file_name(format!("archway-{}-{}", job_info.uuid, folder_num))
                .to_string_lossy()
                .to_string();
            folder_num += 1;
            output_dir_path = std::path::PathBuf::from(output_dir.clone().as_str());
        }

        if folder_num > copies {
            println!(
                "Output directory already exists: {}",
                output_dir_path.display()
            );
            // Rename all folders and then delete oldest
            folder_num = 1;
            while output_dir_path.exists() && folder_num <= copies {
                output_dir = output_dir_path
                    .with_file_name(format!("archway-{}-{}", job_info.uuid, folder_num))
                    .to_string_lossy()
                    .to_string();
                let output_dir_mod = output_dir_path.with_file_name(format!(
                    "archway-{}-{}",
                    job_info.uuid,
                    folder_num - 1
                ));

                fs::rename(&output_dir, &output_dir_mod).unwrap_or_else(|_| {
                    println!(
                        "Failed to rename output directory: {}",
                        output_dir_mod.display()
                    );
                });
                folder_num += 1;
            }

            // Delete the oldest folder if it exists
            let oldest_folder =
                output_dir_path.with_file_name(format!("archway-{}-0", job_info.uuid));
            if oldest_folder.exists() {
                fs::remove_dir_all(&oldest_folder).unwrap_or_else(|_| {
                    println!(
                        "Failed to delete oldest folder: {}",
                        oldest_folder.display()
                    );
                });
            }

            // Ensure set to correct output directory
            output_dir = output_dir_path
                .with_file_name(format!("archway-{}-{}", job_info.uuid, copies))
                .to_string_lossy()
                .to_string();
        }
    }

    output_dir_path = std::path::PathBuf::from(&output_dir);
    if !output_dir_path.exists() {
        match std::fs::create_dir_all(&output_dir) {
            Ok(_) => println!("Created output directory: {}", output_dir),
            Err(e) => {
                println!("Failed to create output directory: {}", e);
                update_job_status(
                    uuid.as_str(),
                    2,
                    String::from("Job failed."),
                    String::from("Failed to create output directory."),
                    false,
                    true,
                    0.0,
                );
                set_job_update(uuid.clone(), "not_running".to_string());
                job_failed_notification(job_info.uuid);

                return;
            }
        }
    }

    tauri::async_runtime::spawn(job_stage_three(uuid, files, output_dir_path));
}

// Stage three of the job: Copying files
async fn job_stage_three(uuid: String, files: Vec<String>, output_dir: PathBuf) {
    let total_files = files.len() as u32;
    let mut output_paths: Vec<String> = Vec::new();
    let mut processed_files = 0;

    update_job_status(
        uuid.as_str(),
        3,
        String::from("Copying Files"),
        String::from("Starting file copy..."),
        true,
        false,
        0.0,
    );

    println!("Output directory: {}", output_dir.display());

    let job_info = storage_manager::get_job_by_uuid(&uuid);
    let input_dirs_struct = job_info.input_dirs.clone();
    let mut input_dirs_cleaned: Vec<String> = Vec::new();

    for input_dir in input_dirs_struct {
        if input_dir.path_type == "library" {
            if input_dir.path == "documents" {
                let documents_path = dirs::document_dir().unwrap();
                input_dirs_cleaned.push(documents_path.to_string_lossy().to_string());
            } else if input_dir.path == "downloads" {
                let downloads_path = dirs::download_dir().unwrap();
                input_dirs_cleaned.push(downloads_path.to_string_lossy().to_string());
            } else if input_dir.path == "desktop" {
                let desktop_path = dirs::desktop_dir().unwrap();
                input_dirs_cleaned.push(desktop_path.to_string_lossy().to_string());
            } else if input_dir.path == "music" {
                let music_path = dirs::audio_dir().unwrap();
                input_dirs_cleaned.push(music_path.to_string_lossy().to_string());
            } else if input_dir.path == "pictures" {
                let pictures_path = dirs::picture_dir().unwrap();
                input_dirs_cleaned.push(pictures_path.to_string_lossy().to_string());
            } else if input_dir.path == "videos" {
                let videos_path = dirs::video_dir().unwrap();
                input_dirs_cleaned.push(videos_path.to_string_lossy().to_string());
            } else {
                println!("Unknown library path: {}", input_dir.path);
            }
        } else {
            input_dirs_cleaned.push(input_dir.path.clone());
        }
    }

    for file in &files {
        if handle_pause_stop(uuid.clone()){
            return;
        }

        let file_path = PathBuf::from(&file);
        let mut file_path_str = file_path.to_string_lossy().to_string();
        update_last_action(
            uuid.as_str(),
            format!(
                "Copying file: {} ({}/{})",
                file_path_str,
                processed_files + 1,
                total_files
            ),
        );

        // Remove the input directory from the file path so the directory structure is preserved
        let longest_matching_dir = input_dirs_cleaned
            .iter()
            .filter(|input_dir| file_path_str.starts_with(*input_dir))
            .max_by_key(|input_dir| input_dir.len());
        if let Some(longest_dir) = longest_matching_dir {
            // Keep the last child directory in the path

            let mut parts: Vec<&str> = Vec::new();

            if longest_dir.contains('\\') {
                parts = longest_dir.split('\\').collect();
            } else if longest_dir.contains('/') {
                parts = longest_dir.split('/').collect();
            }

            // Get the last part of the path
            let last_part = parts.pop().unwrap();

            // Remove the longest matching input directory from the file path
            file_path_str = last_part.to_string() + &file_path_str.replace(longest_dir, "");
        }

        println!("File path: {}", file_path_str);
        println!("Output directory: {}", output_dir.display());

        // Ensure the output directory exists
        let output_file_full_path = output_dir.join(&file_path_str);
        println!("Output file full path: {}", output_file_full_path.display());
        output_paths.push(output_file_full_path.to_string_lossy().to_string());
        let output_file_parent = output_file_full_path.parent();
        if !output_file_parent.as_ref().unwrap().exists() {
            match std::fs::create_dir_all(output_file_parent.as_ref().unwrap()) {
                Ok(_) => println!(
                    "Created output directory: {}",
                    output_file_parent.as_ref().unwrap().display()
                ),
                Err(e) => {
                    println!("Failed to create output directory: {}", e);
                    update_job_status(
                        uuid.as_str(),
                        3,
                        String::from("Job failed."),
                        String::from("Failed to create output directory."),
                        false,
                        true,
                        0.0,
                    );
                    set_job_update(uuid.clone(), "not_running".to_string());
                    job_failed_notification(job_info.uuid);

                    return;
                }
            }
        }

        // Copy the file
        let output_file = output_dir.join(&file_path_str);
        match std::fs::copy(&file, &output_file) {
            Ok(_) => {
                processed_files += 1;
                let percent = processed_files as f32 / total_files as f32;
                update_job_progress(uuid.as_str(), percent);
                update_last_action(
                    uuid.as_str(),
                    format!(
                        "Copied file: {} ({}/{})",
                        file_path_str, processed_files, total_files
                    ),
                );
            }
            Err(e) => {
                println!("Failed to copy file {}: {}", file, e);
                update_job_status(
                    uuid.as_str(),
                    3,
                    String::from("Job failed."),
                    format!("Failed to copy file: {}", file),
                    false,
                    true,
                    0.0,
                );
                set_job_update(uuid.clone(), "not_running".to_string());
                job_failed_notification(job_info.uuid);

                return;
            }
        }
    }
    tauri::async_runtime::spawn(job_stage_four(uuid, files, output_paths));
}

// Stage four of the job: Verifying files
async fn job_stage_four(uuid: String, input_files: Vec<String>, output_files: Vec<String>) {
    update_job_status(
        uuid.as_str(),
        4,
        String::from("Verifying Files"),
        String::from("Verifying copied files..."),
        true,
        false,
        0.0,
    );

    // Ensure input and output files match
    if input_files.len() != output_files.len() {
        println!(
            "Input and output file counts do not match: {} != {}",
            input_files.len(),
            output_files.len()
        );
        update_job_status(
            uuid.as_str(),
            4,
            String::from("Job failed."),
            String::from("Input and output file counts do not match."),
            false,
            true,
            0.0,
        );
        set_job_update(uuid.clone(), "not_running".to_string());
        job_failed_notification(uuid);

        return;
    }

    let mut verified_files = 0;
    let total_files = input_files.len() as u32;
    let mut failed_files: Vec<String> = Vec::new();

    // Iterate through input and output files to verify the hashes match
    for (input_file, output_file) in input_files.iter().zip(output_files.iter()) {
        if handle_pause_stop(uuid.clone()){
            return;
        }
        update_last_action(
            uuid.as_str(),
            format!(
                "Verifying file: {} ({}/{})",
                output_file, verified_files, total_files
            ),
        );
        let input_file_path = PathBuf::from(input_file);
        let output_file_path = PathBuf::from(output_file);

        if !output_file_path.exists() {
            println!("Output file does not exist: {}", output_file);
            failed_files.push(output_file.clone());
            continue;
        }

        match compare_files(
            input_file_path.to_str().unwrap(),
            output_file_path.to_str().unwrap(),
        ) {
            Ok(true) => {
                verified_files += 1;
                let percent = verified_files as f32 / total_files as f32;
                update_job_progress(uuid.as_str(), percent);
                update_last_action(
                    uuid.as_str(),
                    format!(
                        "Verified file: {} ({}/{})",
                        output_file, verified_files, total_files
                    ),
                );
            }
            Ok(false) => {
                println!("File verification failed for: {}", output_file);
                failed_files.push(output_file.clone());
            }
            Err(e) => {
                println!("Error comparing files: {}", e);
                failed_files.push(output_file.clone());
            }
        }
    }

    if failed_files.is_empty() {
        let job_info = storage_manager::get_job_by_uuid(&uuid);
        if job_info.file_behavior == "move" {
            // If moving files, delete the original files
            tauri::async_runtime::spawn(job_stage_five(uuid.clone(), input_files));
        } else {
            println!("All files verified successfully.");
            set_job_update(uuid.clone(), "not_running".to_string());

            update_job_status(
                uuid.as_str(),
                4,
                String::from("Job completed."),
                String::from("All files verified successfully."),
                true,
                true,
                1.0,
            );

            set_job_health_by_uuid(uuid.as_str(), "good");
            get_app_handle()
                .notification()
                .builder()
                .title("Job Complete: ".to_owned() + &job_info.job_name)
                .body("The job has been completed.")
                .show()
                .unwrap();
        }
    } else {
        // Recopy failed files
        println!("Some files failed verification: {:?}", failed_files);

        update_job_status(
            uuid.as_str(),
            4,
            String::from("Job failed."),
            format!("Some files failed verification: {:?}", failed_files),
            false,
            true,
            0.0,
        );

        set_job_update(uuid.clone(), "not_running".to_string());
        job_failed_notification(uuid);
    }
}

// Stage five of the job: Deleting original files (if moving files)
async fn job_stage_five(uuid: String, input_files: Vec<String>) {
    update_job_status(
        uuid.as_str(),
        5,
        String::from("Deleting Original Files"),
        String::from("Deleting original files..."),
        true,
        false,
        0.0,
    );

    let job_info = storage_manager::get_job_by_uuid(&uuid);
    if job_info.file_behavior != "move" {
        println!("Job is not set to move files, skipping deletion.");
        update_job_status(
            uuid.as_str(),
            5,
            String::from("Job completed."),
            String::from("Job is not set to move files, skipping deletion."),
            true,
            true,
            1.0,
        );
        return;
    }

    let total_files = input_files.len() as u32;
    let mut deleted_files = 0;

    for file in input_files {
        if handle_pause_stop(uuid.clone()){
            return;
        }
        update_last_action(
            uuid.as_str(),
            format!(
                "Deleting file: {} ({}/{})",
                file, deleted_files, total_files
            ),
        );
        match std::fs::remove_file(&file) {
            Ok(_) => {
                update_last_action(uuid.as_str(), format!("Deleted file: {}", file));
                deleted_files += 1;
            }
            Err(e) => {
                println!("Failed to delete file {}: {}", file, e);
            }
        }
    }

    set_job_update(uuid.clone(), "not_running".to_string());
    update_job_status(
        uuid.as_str(),
        5,
        String::from("Job completed."),
        String::from("All original files deleted successfully."),
        true,
        true,
        1.0,
    );

    set_job_health_by_uuid(uuid.as_str(), "good");

    get_app_handle()
        .notification()
        .builder()
        .title("Job Complete: ".to_owned() + &job_info.job_name)
        .body("The job has been completed.")
        .show()
        .unwrap();
}

fn job_failed_notification(uuid: String) {
    set_job_health_by_uuid(uuid.as_str(), "bad");

    let job_info = storage_manager::get_job_by_uuid(&uuid);
    get_app_handle()
        .notification()
        .builder()
        .title("Job Failed: ".to_owned() + &job_info.job_name)
        .body("The job has failed. See status for details.")
        .show()
        .unwrap();
}

pub fn get_active_jobs() -> u8 {
    let job_statuses = JOB_STATUSES.lock().unwrap();
    job_statuses.iter().filter(|js| !js.completed).count() as u8
}

fn handle_pause_stop(uuid: String) -> bool {
    if get_job_update(uuid.clone()) == "pause_requested" {
        set_job_update(uuid.clone(), "paused".to_string());
    }
    while get_job_update(uuid.clone()) == "paused" {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    if get_job_update(uuid.clone()) == "stop_requested" {
        set_job_update(uuid.clone(), "not_running".to_string());
        update_job_status(
            &uuid,
            0,
            String::from("Job Stopped"),
            String::from("The job has been stopped by the user."),
            false,
            true,
            0.0,
        );
        job_failed_notification(uuid);
        return true;
    }
    return false;
}
