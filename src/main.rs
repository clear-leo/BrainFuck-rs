use std::{env::args ,fs::File, io::Read};
use brainfuck::handle_error;

const HELP_MSG: &str = r#"USAGE: brainfuck [-vh] [file]
    -v --version             Shows version information
    -h --help                Shows this help message"#;


fn main() {
    let version_msg = format!(r#"VERSION: {} {}
MIT OR Apache-2.0 clearleo 2024"#, brainfuck::VERSION, brainfuck::UPDATE_DATE);

    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        brainfuck::interpret_ui();
        return;
    }
    match args[1].to_lowercase().trim() {
        "--help" | "-h" => {
            println!("{HELP_MSG}");
            return;
        }
        "--version" | "-v" => {
            println!("{version_msg}");
            return;
        }
        _ => (),
    }    
    
    let mut file = handle_error!(File::open(&args[1]));
    let mut file_contents = String::new();
    handle_error!(file.read_to_string(&mut file_contents));
    brainfuck::run_str(&file_contents);
}
