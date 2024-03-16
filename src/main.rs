use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    let mut f = File::open("file.txt")?;
    let mut source = vec![];

    f.read_to_end(&mut source);
}