#![allow(unused)]

use std::process::Command;
use indicatif::ProgressBar;
use clap::Parser;
use reqwest::Client;
use std::path::Path;
use std::io::Cursor;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {

    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");

    command()
}

fn command() {

    progress();

       // check local cache for specified ruby-wasm build.
    if !Path::new("/etc/hosts").exists() {
        println!("exists");
    } else { // make it
        get("https://github.com/ruby/ruby.wasm/releases/download/2022-06-01-a/ruby-head-wasm32-unknown-wasi-full.tar.gz".to_string(), "wasm-ruby-latest.gz".to_string());
        Command::new("tar xfz wasm-ruby-latest.tar.gz")
            .status()
            .expect("ls command failed to start");
        Command::new("mv head-wasm32-unknown-wasi-full/usr/local/bin/ruby ruby.wasm")
            .status()
            .expect("ls command failed to start");

        Command::new("mkdir src; echo 'puts 'hello'' > src/my_app.rb")
            .status()
            .expect("ls command failed to start");

        Command::new("wasi-vfs pack ruby.wasm --mapdir /src::./src --mapdir /usr::./head-wasm32-unknown-wasi-full/usr -o my-ruby-app.wasm")
            .status()
            .expect("ls command failed to start");
        }
}

fn get(url: String, file_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut res = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(res.bytes()?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

fn progress() {
    let pb = indicatif::ProgressBar::new(10000);
    for i in 0..10000{
        //pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
