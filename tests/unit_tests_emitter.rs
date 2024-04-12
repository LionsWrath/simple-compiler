#[path = "../src/lex.rs"] mod lex;
#[path = "../src/parse.rs"] mod parse;
#[path = "../src/emitter.rs"] mod emitter;
#[path = "../src/utils/utils.rs"] mod utils;

use std::path::PathBuf;
use utils::read_file;

fn compile(filename: &str) {

    let filepath = PathBuf::from(filename);
    let file = String::from(filepath.file_stem().unwrap().to_str().unwrap()) + ".c";

    let mut parser = parse::Parser::new(
        lex::Lexer::new(utils::read_file(&filepath)),
        emitter::Emitter::new(PathBuf::from(file)),
    );

    parser.emit();
}

fn compare_files(filename: &str) -> bool {

    let filepath = PathBuf::from(filename);
   
    let o = read_file(&PathBuf::from(filepath.file_name().unwrap()));
    let i = read_file(&filepath);

    o == i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        compile("src/examples/code/source/average.tb");
        assert!(compare_files("src/examples/code/built/average.c"));
    }

    #[test]
    fn test_fibonacci() {
        compile("src/examples/code/source/fibonacci.tb");
        assert!(compare_files("src/examples/code/built/fibonacci.c"));
    }
}
