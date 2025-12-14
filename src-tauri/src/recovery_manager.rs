use crate::storage_manager;
use crate::storage_manager::to_json_string;

pub fn verify_recovery_file(file_path: &str) -> &str {
    let file_path = file_path.replace("\\", "/");

    println!("Verifying recovery file at path: {}", file_path);
    // Replace all backslashes with forward slashes for consistency
    return match storage_manager::read_json_file::<Vec<Vec<String>>>(
        file_path.to_string()
    ) {
        Ok(recovery_data) => {
            // Check that all inner arrays have exactly two string elements
            for entry in recovery_data.iter() {
                if entry.len() != 2 {
                    return "Error: Each entry in the recovery file must contain exactly two string elements.";
                }
            }

            "Valid"
        }
        Err(e) => {
            println!("Error reading recovery file: {}", e);
            "Error: Recovery file is not JSON or not in the correct format."
        },
    }
}


pub fn get_recovery_file(file_path: &str) -> String {
    let file_path = file_path.replace("\\", "/");

    println!("Verifying recovery file at path: {}", file_path);
    // Replace all backslashes with forward slashes for consistency
    return match storage_manager::read_json_file::<Vec<Vec<String>>>(
        file_path.to_string()
    ) {
        Ok(recovery_data) => {
            // Check that all inner arrays have exactly two string elements
            for entry in recovery_data.iter() {
                if entry.len() != 2 {
                    return "Error: Each entry in the recovery file must contain exactly two string elements.".to_string();
                }
            }

            to_json_string(&recovery_data)
        }
        Err(e) => {
            println!("Error reading recovery file: {}", e);
            "Error: Recovery file is not JSON or not in the correct format.".to_string()
        },
    }
}