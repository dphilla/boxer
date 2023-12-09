use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_Dockerfile>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    // Define a mapping of Dockerfile directives to their descriptions
    let directives = get_directives_mapping();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                match parse_dockerfile_line(&ip, &directives) {
                    Ok(description) => println!("{}", description),
                    Err(e) => return Err(e),
                }
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

fn parse_dockerfile_line(line: &str, directives: &HashMap<String, String>) -> Result<String, io::Error> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        // Skip empty lines and comments
        return Ok(String::new());
    }

    if let Some(directive) = line.split_whitespace().next() {
        match directives.get(directive) {
            Some(description) => Ok(description.to_string()),
            None => Err(io::Error::new(io::ErrorKind::Other, format!("Directive {} not recognized", directive))),
        }
    } else {
        Ok(String::new())
    }
}

fn get_directives_mapping() -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert("FROM".to_string(), "FROM: ()Initializes a new build stage and sets the base layer.".to_string());
    map.insert("RUN".to_string(), "RUN: Executes commands in a new layer on top of the current layer and commits the results.".to_string());
    map.insert("CMD".to_string(), "CMD: Provides defaults for an executing container. Only one CMD instruction in a Dockerfile.".to_string());
    map.insert("LABEL".to_string(), "LABEL: Adds metadata to a layer as key-value pairs.".to_string());
    map.insert("EXPOSE".to_string(), "EXPOSE: Informs Docker that the container listens on specified network ports at runtime.".to_string());
    map.insert("ENV".to_string(), "ENV: Sets environment variables.".to_string());
    map.insert("ADD".to_string(), "ADD: Copies new files, directories, or remote file URLs and adds them to the layer's filesystem.".to_string());
    map.insert("COPY".to_string(), "COPY: Copies new files or directories and adds them to the filesystem of the container.".to_string());
    map.insert("ENTRYPOINT".to_string(), "ENTRYPOINT: Configures a container that will run as an executable.".to_string());
    map.insert("VOLUME".to_string(), "VOLUME: Creates a mount point with the specified name.".to_string());
    map.insert("USER".to_string(), "USER: Sets the username or UID for running the layer and following RUN, CMD, and ENTRYPOINT instructions.".to_string());
    map.insert("WORKDIR".to_string(), "WORKDIR: Sets the working directory for RUN, CMD, ENTRYPOINT, COPY, and ADD instructions.".to_string());
    map.insert("ARG".to_string(), "ARG: Defines a variable that users can pass at build-time to the builder.".to_string());
    map.insert("ONBUILD".to_string(), "ONBUILD: Adds a trigger instruction to be executed when the layer is used as the base for another build.".to_string());
    map.insert("STOPSIGNAL".to_string(), "STOPSIGNAL: Sets the system call signal that will be sent to the container to exit.".to_string());
    map.insert("HEALTHCHECK".to_string(), "HEALTHCHECK: Tells Docker how to test a container to check that it is still working.".to_string());
    map.insert("SHELL".to_string(), "SHELL: Allows the default shell used for the shell form of commands to be overridden.".to_string());
    map.insert("MAINTAINER".to_string(), "MAINTAINER: (Deprecated) Used to set the author field of the generated layer.".to_string());

    map
}

