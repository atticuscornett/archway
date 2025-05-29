use crate::structs::JobInfo;

pub fn to_json_string<T: serde::Serialize>(data: &T) -> String {
    match serde_json::to_string(data) {
        Ok(json) => json,
        Err(err) => {
            println!("Error serializing to JSON: {}", err);
            String::new()
        }
    }
}

pub fn from_json_string<T: serde::de::DeserializeOwned>(json: String) -> Result<T, serde_json::Error> {
    serde_json::from_str(&json)
}

pub fn read_json_file<T: serde::de::DeserializeOwned>(file_path: String) -> Result<T, std::io::Error> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

pub fn write_json_file<T: serde::Serialize>(file_path: String, data: &T) -> Result<(), std::io::Error> {
    let file = std::fs::File::create(file_path)?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer(writer, data).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

pub fn file_with_executable(file: &str) -> String{
    let mut executable_path = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
    executable_path = executable_path.join(file).to_path_buf();
    executable_path.to_string_lossy().to_string()
}

pub fn set_all_jobs(all_jobs: Vec<JobInfo>) -> bool {
    write_json_file(file_with_executable("jobs.json"), &all_jobs).is_ok()
}

pub fn get_all_jobs() -> Vec<JobInfo> {
    match read_json_file::<Vec<JobInfo>>(file_with_executable("jobs.json")) {
        Ok(jobs) => jobs,
        Err(err) => {
            println!("Error reading jobs file: {}", err);
            Vec::new()
        }
    }
}

pub fn get_job_by_uuid(uuid: &str) -> JobInfo {
    let all_jobs = get_all_jobs();
    all_jobs.into_iter().find(|job| job.uuid == uuid).expect("Job not found")
}

pub fn remove_job_by_uuid(uuid: &str) -> bool {
    let mut all_jobs = get_all_jobs();
    if let Some(pos) = all_jobs.iter().position(|job| job.uuid == uuid) {
        all_jobs.remove(pos);
        set_all_jobs(all_jobs)
    } else {
        println!("Job with UUID {} not found", uuid);
        false
    }
}