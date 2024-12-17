use std::{env::args ,fs::File, io::Read};


macro_rules! handle_error {
    ($function:expr) => {
        match $function {
            Ok(good) => good,
            Err(error) => {println!("brainfck: {}", error); return}
        }
    };
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        brainfck::interpret_ui();
        return;
    }
    let mut file = handle_error!(File::open(&args[1]));
    let mut file_contents = String::new();
    handle_error!(file.read_to_string(&mut file_contents));
    brainfck::run_str(&file_contents);
}
