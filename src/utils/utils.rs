use std::fs::File;
use std::io::Read;

pub fn read_file(filename: &str) -> Vec<char> {
    let mut f = match File::open(filename) {
        Ok(file) => file,
        Err(err) => panic!("[UTILS] {err}"),
    };

    let mut raw: Vec<u8> = Vec::new();
    let _ = f.read_to_end(&mut raw);
    
    raw.iter().map(|b| *b as char).collect::<Vec<_>>()
}
