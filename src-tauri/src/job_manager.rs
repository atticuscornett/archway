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
        total_steps: 4,
        step_title: String::from("Initializing Job"),
        last_action: String::from("Starting job..."),
        success: true,
        percent: 0.0
    };

    JOB_STATUSES.lock().unwrap().push(new_job_status);



    true

}

pub fn get_all_job_statuses() -> Vec<JobStatus> {
    JOB_STATUSES.lock().unwrap().clone()
}