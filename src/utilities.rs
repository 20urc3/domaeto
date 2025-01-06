use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Write;


pub fn create_folder(path: &str) -> std::io::Result<()> {
    let path = Path::new(path);
    if path.exists() {
        let total_size: u64 = fs::read_dir(path)? // Calculate total size of directory in bytes
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.metadata().ok())
            .map(|metadata| metadata.len())
            .sum();
        let size_mb = total_size as f64 / 1_048_576.0; // Convert bytes to megabytes (1 MB = 1,048,576 bytes)
        if size_mb > 2.0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("The path provided for output directory already contains results and will not be overwritten")
            ));
        }
    }
    fs::create_dir(path)
}

pub fn write_sample_to_file(path: &str, file_number: u32, sample: String) -> std::io::Result<()> {
    let path = Path::new(path);
    if !path.exists() || !path.is_dir() { // First, check if the provided path is a valid directory
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Provided path is not a valid directory"
        ));
    }

    let filename = format!("sample_{}.html", file_number); // Create the numbered filename (e.g., "sample_1.html")
    let full_path = path.join(filename); // Combine the directory path with the filename
    let mut file = File::create(full_path)?; // Create file
    file.write_all(sample.as_bytes())?; // Write to file
    Ok(())
}

pub fn read_grammar_file(path: &str) -> std::io::Result<String> {
    let grammar_content = std::fs::read_to_string(Path::new(path))?; // Read grammar file and pass content to grammar_content 
    Ok(grammar_content) // Return the content as a String
}