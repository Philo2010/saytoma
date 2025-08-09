//File that reads a file and outputs a struct to read any file by page number
use std::fs::File;
use std::io::{BufReader, BufWriter, copy};
use std::path::PathBuf;
use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};
use std::sync::Arc;
use std::usize;
use iced::futures::channel::mpsc;
use iced::futures::{SinkExt, Stream};
use tempfile::TempDir;
use zip::ZipArchive;
use tokio::sync::watch;

use crate::Message;


pub enum PageReaderState  {
    None,
    Some(Result<PageReader, std::io::Error>),
    Loading
}


pub struct PageReader {
    //TODO: Fill Data
    pub dir: TempDir,
    pub paths: Vec<PathBuf>,
}

impl PageReader {

    pub async fn new(file: File, tx: Arc<AtomicUsize>) -> Result<PageReader, std::io::Error> {
        let reader = BufReader::new(file); 
        let mut archive = ZipArchive::new(reader)?;
        let temp_dir = tempfile::tempdir()?;
        let mut extracted_paths = Vec::new();
        let invese = (1 as f32/archive.len() as f32)*100 as f32;

        for i in 0..archive.len() {
            let mut file_in_zip = archive.by_index(i)?;

            let out_path = temp_dir.path().join(file_in_zip.name());
            if file_in_zip.name().ends_with('/') {
                std::fs::create_dir_all(&out_path)?;
            } else {
                let out_file = File::create(&out_path)?;
                let mut writer = BufWriter::new(out_file);
                copy(&mut file_in_zip, &mut writer)?;
                extracted_paths.push(out_path);
            }

            tx.store((invese*i as f32) as usize, Ordering::SeqCst);
        }
        extracted_paths.sort_by(|a, b| natord::compare(&a.to_string_lossy(), &b.to_string_lossy()));
        Ok(PageReader { dir: temp_dir, paths:extracted_paths})
    }
    pub fn read_at(&self, index: usize) -> &PathBuf {
        &self.paths[index]
    }
}