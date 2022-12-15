use std::io::{Read, Write};
use std::fs::{File, write};
pub struct FileDatabase{
    fname: String
}
// Mandatory requirement for sub-impl
impl FileDatabase{
    pub fn new(fx: &str) -> FileDatabase{
        if !std::path::Path::new(fx).exists(){
            File::options()
            .write(true)
            .create(true)
            .open(fx)
            .expect("Creation failed.");
        }
        FileDatabase { fname: (String::from(fx)) }
    }

    pub fn read_data(&self, buf: &mut String){
        let mut fc = File::open(&self.fname).expect("Open failed");
        let mut buffer = String::new();
        fc.read_to_string(&mut buffer).expect("Failed to read");
        *buf = buffer.to_owned();
    }
    pub fn data_exist(&self, key: &str) -> bool{
        let mut data = String::new();
        let mut exist = false;
        self.read_data(&mut data);
        for i in data.lines(){
            if i.to_lowercase().contains(&key.to_lowercase()){
                exist = true;
            }
        }
        exist
    }
    pub fn write(&self, content: &str, append: bool) -> Option<&str>{
        if append{
            let mut wrs = File::options().append(true).create(true).open(&self.fname).expect("Failed to open");
            wrs.write(content.as_bytes()).expect("Failed to write");
        }else{
            write(&self.fname, content).expect("Failed to write");
        }
        Some("SC1")
    }
    pub fn getpasswd(&self, key: &str, out: &mut String) -> Option<&str>{
        let mut data = "LIT";
        let mut buf = String::new();
        self.read_data(&mut buf);
        for i in buf.lines(){
            if i.to_lowercase().contains(&key.to_lowercase()){
                let spl = i.split(":").collect::<Vec<&str>>();
                data = spl[1].clone();
            }
        }
        *out = String::from(data);
        Some("SC1")
    }
}
