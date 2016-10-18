use super::journal;

use std::io::{self, Read, Seek};
use std::fs::{OpenOptions, File};
use std::process::Command;
use std::ffi::OsString;
extern crate tempfile;


pub trait Composer {
    fn compose(&self) -> io::Result<String>;
}

impl Composer for str {
    fn compose(&self) -> io::Result<String> {
        Ok(self.to_string())
    }
}


pub struct StdIn;

impl Composer for StdIn {
    fn compose(&self) -> io::Result<String> {
        println!("Type your entry below: ");
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        return Ok(buffer);
    }
}

pub struct Editor(OsString);


impl Composer for Editor {
    fn compose(&self) -> io::Result<String> {
        
        let file = tempfile::NamedTempFile::new().expect("Could not create temp file");
        Command::new(&self.0)
            .arg(file.path().as_os_str())
            .spawn()
            .expect("Could not spawn editor")
            .wait()
            .unwrap();
        let mut f: File = file.into();
        f.seek(io::SeekFrom::Start(0)).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("Read error!");
        return Ok(buf);
    }
}
