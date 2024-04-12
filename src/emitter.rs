use std::fs::File;
use std::io::prelude::*;
use std::panic;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Emitter {
    pub full_path: std::path::PathBuf,
    pub header: String,
    pub code: String,
}

impl Emitter {

    pub fn new(full_path: PathBuf) -> Self {

        let header = "".to_string();
        let code = "".to_string();

        Emitter {
            full_path,
            header,
            code,
        }
    }

    pub fn emit(&mut self, code: &str) {
        self.code += code;
    }

    pub fn emit_line(&mut self, code: &str) {
        self.code += code;
        self.code += "\n";
    }

    pub fn header(&mut self, code: &str) {
        self.header += code;
    }

    pub fn header_line(&mut self, code: &str) {
        self.header += code;
        self.header += "\n";
    }

    #[allow(dead_code)]
    pub fn write_file(&self) {
        let mut f = match File::create(&self.full_path) {
            Ok(file) => file,
            Err(err) => panic!("[EMITTER] {err}"),
        };

        match f.write_all(self.header.as_bytes()) {
            Ok(_) => (),
            Err(err) => panic!("[EMITTER] {err}"),
        };

        match f.write_all(self.code.as_bytes()) {
            Ok(_) => (),
            Err(err) => panic!("[EMITTER] {err}"),
        };
    }
}
