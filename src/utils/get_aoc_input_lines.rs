use std::{io::{self, BufRead}, fs::File, env};

pub fn get_input_file_path_from_args() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a path to the input file as an argument");
        std::process::exit(1);
    }

    let file_path = &args[1];
    file_path.to_string()
}

pub fn get_aoc_input_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file_path = get_input_file_path_from_args();
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}