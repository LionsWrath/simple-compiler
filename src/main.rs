mod lex;
mod parse;
mod emitter;
#[path = "utils/utils.rs"] mod utils;

use clap::{Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_filename: std::path::PathBuf,
    #[arg(short, long, default_value = "out.c")]
    output_filename: std::path::PathBuf,
}

fn main() {
    println!("Teeny Tiny Compiler");

    let args = Args::parse();

    let mut parser = parse::Parser::new(
        lex::Lexer::new(utils::read_file(&args.input_filename)),
        emitter::Emitter::new(args.output_filename),
    );

    parser.emit();
    println!("Compiling Completed!");
}
