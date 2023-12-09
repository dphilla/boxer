use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_Dockerfile>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                parse_dockerfile_line(&ip);
            }
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_dockerfile_line(line: &str) {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        // Skip empty lines and comments
        return;
    }

    if let Some((directive, command)) = line.split_once(' ') {
        println!("Directive: {}, Command: {}", directive, command);
    }
}
