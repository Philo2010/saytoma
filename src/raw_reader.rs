//File that reads a file and outputs a struct to read any file by page number
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use tempfile::tempdir;
use zip::ZipArchive;

struct PageReader {
    //TODO: Fill Data
}

impl PageReader {
    pub fn new(path: &Path) {
        
    }
}