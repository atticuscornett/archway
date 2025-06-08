use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn log(log: &str, body: &str, msg_type: &str) {
    let log_file_path = crate::storage_manager::file_with_executable(&*("archway-".to_owned() + log + ".log"));
    let local_time = time::OffsetDateTime::now_local()
        .unwrap_or_else(|_| time::OffsetDateTime::now_utc())
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "unknown time".to_string());

    let log_entry = format!("{} [{}]: {}\n", local_time, msg_type, body);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file_path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", log_entry) {
        eprintln!("Failed to write to log file: {}", e);
    }
}

pub fn log_info(log_title: &str, body: &str) {
    log(log_title, body, "INFO");
}

pub fn log_error(log_title: &str, body: &str) {
    log(log_title, body, "ERROR");
}

pub fn job_log(job_id: &str, body: &str, msg_type: &str, log_setting: String) {
    let log_title = "job-".to_owned() + job_id;
    let log_title_str = log_title.as_str();
    if msg_type == "FILE" {
        if log_setting == "high"{
            log_info(log_title_str, body);
        }
        return;
    }
    if msg_type == "STEP" {
        if log_setting != "low" {
            log_info(log_title_str, body);
        }
        return;
    }
    if msg_type == "ERROR" {
        log_error(log_title_str, body);
        return;
    }
    else {
        log_info(log_title_str, body);
    }
}