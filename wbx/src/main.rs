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

    let directives = get_directives_mapping();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let directive = ip.split_whitespace().next().unwrap_or_default().to_uppercase();
                if let Some(execute) = directives.get(&directive) {
                    execute();
                } else if !directive.is_empty() {
                    eprintln!("Directive {} not recognized", directive);
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

fn get_directives_mapping() -> HashMap<String, Box<dyn Fn()>> {
    let mut map = HashMap::new();

    map.insert("FROM".to_string(), Box::new(execute_from));
    map.insert("RUN".to_string(), Box::new(execute_run));
    map.insert("CMD".to_string(), Box::new(execute_cmd));
    map.insert("LABEL".to_string(), Box::new(execute_label));
    map.insert("EXPOSE".to_string(), Box::new(execute_expose));
    map.insert("ENV".to_string(), Box::new(execute_env));
    map.insert("ADD".to_string(), Box::new(execute_add));
    map.insert("COPY".to_string(), Box::new(execute_copy));
    map.insert("ENTRYPOINT".to_string(), Box::new(execute_entrypoint));
    map.insert("VOLUME".to_string(), Box::new(execute_volume));
    map.insert("USER".to_string(), Box::new(execute_user));
    map.insert("WORKDIR".to_string(), Box::new(execute_workdir));
    map.insert("ARG".to_string(), Box::new(execute_arg));
    map.insert("ONBUILD".to_string(), Box::new(execute_onbuild));
    map.insert("STOPSIGNAL".to_string(), Box::new(execute_stopsignal));
    map.insert("HEALTHCHECK".to_string(), Box::new(execute_healthcheck));
    map.insert("SHELL".to_string(), Box::new(execute_shell));
    map.insert("MAINTAINER".to_string(), Box::new(execute_maintainer));

    map
}

fn execute_from() {
    println!("Executing FROM directive");
}

fn execute_run() {
    println!("Executing RUN directive");
}

fn execute_cmd() {
    println!("Executing CMD directive");
}

fn execute_label() {
    println!("Executing LABEL directive");
}

fn execute_expose() {
    println!("Executing EXPOSE directive");
}

fn execute_env() {
    println!("Executing ENV directive");
}

fn execute_add() {
    println!("Executing ADD directive");
}

fn execute_copy() {
    println!("Executing COPY directive");
}

fn execute_entrypoint() {
    println!("Executing ENTRYPOINT directive");
}

fn execute_volume() {
    println!("Executing VOLUME directive");
}

fn execute_user() {
    println!("Executing USER directive");
}

fn execute_workdir() {
    println!("Executing WORKDIR directive");
}

fn execute_arg() {
    println!("Executing ARG directive");
}

fn execute_onbuild() {
    println!("Executing ONBUILD directive");
}

fn execute_stopsignal() {
    println!("Executing STOPSIGNAL directive");
}

fn execute_healthcheck() {
    println!("Executing HEALTHCHECK directive");
}

fn execute_shell() {
    println!("Executing SHELL directive");
}

fn execute_maintainer() {
    println!("Executing MAINTAINER directive");
}
