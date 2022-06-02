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

    // 1. understand context (lang ecosystem), and have relevant tooling downloaded for that
    //     - can also pass in if used outside of language package environment, i.e. just installed
    //     or built from source (eventually, would be great to do wasm-releases as the default):
    //     this should be walked through, in the nice waw vue-cli or rust installation works :)
    //

    // # Download a prebuilt Ruby release
    //$ curl -LO https://github.com/kateinoigakukun/ruby.wasm/releases/download/2022-03-28-a/ruby-head-wasm32-unknown-wasi-full.tar.gz
    //$

    //  check if exists locally, in some kinda cache or something, else

    progress();

    // todo: check local cache for built wasm ruby
    if !Path::new("/etc/hosts").exists() {
      println!("exists");
    } else {
      get("https://github.com/ruby/ruby.wasm/releases/download/2022-06-01-a/ruby-head-wasm32-unknown-wasi-full.tar.gz".to_string(), "wasm-ruby-latest.gz".to_string());
    }

    //Command::new("ruby")
        //.arg("-e")
        //.arg("puts 'hi'")
        //.status()
        //.expect("ls command failed to start");

    Command::new("tar xfz wasm-ruby-latest.tar.gz")
    //# Extract ruby binary not to pack itself
    //$ mv  ruby.wasm
    Command::new("mv head-wasm32-unknown-wasi-full/usr/local/bin/ruby ruby.wasm")

    // app source code for test
    Command::new("mkdir src; echo "'puts 'hello'" > src/my_app.rb")

    //# Pack the whole directory under /usr and your app dir
    Command::new("wasi-vfs pack ruby.wasm --mapdir /src::./src --mapdir /usr::./head-wasm32-unknown-wasi-full/usr -o my-ruby-app.wasm")

    // combine all the steps above, and then add Gem bundling mechanism
    //   - needs to detect with type/version of ruby is installed in dir, and get matching prebuilt binary ?
    //
    // 3. make bundling work bundle: https://gist.github.com/kateinoigakukun/5caf3b83b2732b1653e91b0e75ce3390
    //
    // 4. Make built application split out at /wasm-dist/<entrypoint-script>.wasm
    //
    // 5. able to do `wbx run <entrypoint-script>.wasm` which invokes either the default or specified runtime (wasmtime)
    //
    // Entire rubywasm + source code pushed to registry (possibly, bindle)
}

fn get(url: String, file_name: String) -> Result<(), Box<dyn std::error::Error>> {

    let mut res = reqwest::blocking::get(url)?;

    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(res.bytes()?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())

    //println!("Status: {}", res.status());
    //println!("Headers:\n{:?}", res.headers());

    //res.copy_to(&mut std::io::stdout())?;

    //println!("\n\nDone.");
    //Ok(())
}

fn progress() {
    let pb = indicatif::ProgressBar::new(10000);
    for i in 0..10000{
        //pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done, but cant see this shit");
}
