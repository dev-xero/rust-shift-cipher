use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write_result(result: String, shift: &i8) -> Result<(), String> {
    let file_path = format!("./encrypted/decryption_key_{result}-{shift}.txt");
    let parent_dir = Path::new(&file_path).parent().unwrap();
    
    if !parent_dir.exists() {
        match std::fs::create_dir_all(parent_dir) {
            Ok(_) => println!("Created directories successfully."),
            Err(e) => return Err(format!("Error creating directory: {}", e))
        }
    }

    let mut file = match File::create(&file_path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to write file to path: {}, error: {}", file_path, e))
    };

    match file.write_all(result.as_bytes()) {
        Ok(_) => {
            println!("File written successfully to {}", file_path);
            return Ok(())
        },
        Err(e) => return Err(format!("Error writing to file: {}", e)),
    }
}