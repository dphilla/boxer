#![allow(unused)]

use std::process::Command;
use indicatif::ProgressBar;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {

    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");

    for line in content.lines() {
       println!("{}", line);
    }

    progress();
    command();
}

fn progress() {
    let pb = indicatif::ProgressBar::new(10000);
    for i in 0..10000{
        //pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done, but cant see this shit");
}

fn command() {
    //Command::new("ruby")
        //.arg("-e")
        //.arg("puts 'hi'")
        //.status()
        //.expect("ls command failed to start");

    // 1. understand context (a pip lib, or an gem ), and have relevant tooling downloaded for that
    //     - can also pass in if used outside of language package environment, i.e. just installed
    //     or built from source (eventually, would be great to do wasm-releases as the default):
    //     this should be walked through, in the nice waw vue-cli or rust installation works :)
    //
    // 2.  # Download a prebuilt Ruby release
    // # Download a prebuilt Ruby release
            //$ curl -LO https://github.com/kateinoigakukun/ruby.wasm/releases/download/2022-03-28-a/ruby-head-wasm32-unknown-wasi-full.tar.gz
            //$ tar xfz ruby-head-wasm32-unknown-wasi-full.tar.gz

            //# Extract ruby binary not to pack itself
            //$ mv head-wasm32-unknown-wasi-full/usr/local/bin/ruby ruby.wasm

            //# Put your app code
            //$ mkdir src
            //$ echo "puts 'Hello'" > src/my_app.rb

            //# Pack the whole directory under /usr and your app dir
            //$ wasi-vfs pack ruby.wasm --mapdir /src::./src --mapdir /usr::./head-wasm32-unknown-wasi-full/usr -o my-ruby-app.wasm
            //
            //# Run the packed scripts
            //$ wasmtime my-ruby-app.wasm -- /src/my_app.rb
            //Hello
    //
    //
    //   - needs to detect with type of ruby is installed in dir, and get matching prebuilt binary ?
    //
    // 3. make bundling work bundle: https://gist.github.com/kateinoigakukun/5caf3b83b2732b1653e91b0e75ce3390
    //
    // 4. Make built application split out at /wasm-box/<entrypoint-script>.wasm
    //
    // 5. able to do `wbx run <entrypoint-script>.wasm` which invokes either the default or specified runtime (wasmtime)

    //let script_path = "tests/sample.rb";

    //Command::new("ruby")
        //.arg(script_path)
        //.status()
        //.expect("ls command failed to start");
        //.arg("puts 'hi'")
        //
    //println!("\n {} compiled to WebAssembly at ./wasm_box/what_to_call_this.wasm", script_path);
    //println!("\n Note: this has been compiled AOT to run server-side, using WASI to manage system calls (more info: https://wasi.dev). Running elsewhere (like in a browser) may not be the best experience with this compilation");
   // println!("\n Execute your compiled {} by typing ./wasm_box/what_to_call_this.wasm (and hitting enter) from your current directory\n", script_path);
}

//ideas from wasm-boxer repo
//`wbx script.rb` aliases wbx build <script>

//`wbx script.rb --web`

//`wbx script.rb --runtime wasmtime` // or choose any other runtime (this makes sure runtime is installed at least)

//Ex. a Rails app:

//`wbx .`

// wbx run /built.wasm or just cd into ./build/ and do wbx run (obv this is kinda like docker run)

//show difference in system usage/size, etc. once you do this

//### Box up your project to run in almost any environment.

//`Default `wbx` compiles AOT to Wasm (no runtime needed -- HOPEFULLY). The output is a .wasm file that can run on any [WebAssembly Runtime](https://github.com/appcypher/awesome-wasm-runtimes), or be Ahead-of-Time compiled.

//show benchmarks compared to containers vms: start up times, size, native vs wasm excution, etc. tons and tons of good visualizations.


