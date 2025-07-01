//File that reads a file and outputs a struct to read any file by page number
use std::fs::File;
use std::io::{BufReader, BufWriter, copy};
use std::path::PathBuf;
use tempfile::TempDir;
use zip::ZipArchive;
struct PageReader {
    //TODO: Fill Data
    dir: TempDir,
    paths: Vec<PathBuf>
}

impl PageReader {
    pub fn new(file: &File) -> Result<PageReader, std::io::Error> {
        let reader = BufReader::new(file); 
        let mut archive = ZipArchive::new(reader)?;

        let temp_dir = tempfile::tempdir()?;
        let mut extracted_paths = Vec::new();

        for i in 0..archive.len() {
            let mut file_in_zip = archive.by_index(i)?;

            let out_path = temp_dir.path().join(file_in_zip.name());

            if file_in_zip.name().ends_with('/') {
                std::fs::create_dir_all(&out_path);
            } else {
                if let Some(parent) = out_path.parent() {
                    std::fs::create_dir_all(&out_path);
                }
                let out_file = File::create(&out_path)?;
                let mut writer = BufWriter::new(out_file);
                copy(&mut file_in_zip, &mut writer)?;
                extracted_paths.push(out_path);
            }
        }

        Ok(PageReader { dir: temp_dir, paths: extracted_paths })
    }
}