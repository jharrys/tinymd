use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    return the_title;
}

fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", _filename);

    let input_filename= Path::new(_filename);
    let file = File::open(&input_filename).expect("[ ERROR ] Failed to open file!");
    // expect unwraps the "Result" type; below is the verbose way
    /*
    let _file = match File::open(&input_filename) {
        Err(err) => panic!("Couldn't open file: {}", err.to_string()),
        Ok(value) => value,
    };
     */

    // Rust will warn/complain about unused variables - prefixing with _ silences this.
    let mut _ptag: bool = false;
    let mut _htag: bool = false;

    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let  line_contents = line.unwrap();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        let mut output_line = String::new();

        match first_char.pop() {
            Some('#') => {},
            _ => {}
        }
    }
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!("Written by: {}\nHomepage: {}\nUsage: tinymd <somefile>.md\n", env!("CARGO_PKG_AUTHORS"), env!("CARGO_PKG_HOMEPAGE"));
}

fn usage() {
    print_long_banner();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid invocation (you done goofed!)");
            usage();
        }
    }
}
