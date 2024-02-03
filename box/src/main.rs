use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::process::Command;
use dockerfile_parser::*;
use structopt::StructOpt;

mod builder {
    pub mod builder;
    pub mod packer;
}

use builder::builder::*;

#[derive(StructOpt)]
#[structopt(name = "boxer", about = "A CLI for building, running, managing (Wasm) boxes.\nMore info at https://boxer.dev")]
enum BoxCli {
    #[structopt(name = "build")]
    /// Builds box from Dockerfile (-f) or OCI Image (-i)
    Build {
        #[structopt(short = "f", long = "file", parse(from_os_str))]
        /// Dockerfile path
        dockerfile_path: std::path::PathBuf,
    },
    #[structopt(name = "run")]
    /// Runs a built box; if no name given, defaults to last built box
    Run {
        // add args to pass runtime, etc
    },
}

fn main() {
    let opt = BoxCli::from_args();

    let mut builder = Builder::new();
    match opt {
        BoxCli::Build { dockerfile_path } => {
            let dockerfile_content = std::fs::read_to_string(dockerfile_path);
            let dockerfile = Dockerfile::parse(&dockerfile_content.unwrap());
            for stage in dockerfile.expect("Error").iter_stages() {
              for ins in stage.instructions {
                match ins {
                    Instruction::From(instr) => execute_from(&mut builder, instr.clone()),
                    Instruction::Arg(instr) => execute_arg(instr.clone()),
                    Instruction::Label(instr) => execute_label(instr.clone()),
                    Instruction::Run(instr) => execute_run(instr.clone()),
                    Instruction::Entrypoint(instr) => execute_entrypoint(instr.clone()),
                    Instruction::Copy(instr) => execute_copy(&mut builder, instr.clone()),
                    Instruction::Cmd(instr) => execute_cmd(instr.clone()),
                    Instruction::Env(instr) => execute_env(instr.clone()),
                    Instruction::Misc(instr) => execute_misc(&mut builder, instr.clone()),
                }
              }

              builder.build(true);
            }
        }
        BoxCli::Run { /* ... */ } => {
            let wasm_file = "final_build.wasm";
            //let script = "demo_src/my_app.rb";
            let script = "src/my_app.rb";

            if let Err(e) = execute_wasm_with_wasmtime(wasm_file, script) {
                eprintln!("Error executing WASM with Wasmtime: {}", e);
            }
        },
    }
}

fn execute_from(builder: &mut Builder, instr: FromInstruction) {
    builder.config_base(&instr.image.content);
}

fn execute_run(instr: RunInstruction) {
  unimplemented!(
        r#"
    handles RUN
    "#
  )
}

fn execute_cmd(instr: CmdInstruction) {
    // start here:

    //     1.2 - save WORKDIR in struct and add it to 1.4 if it is defined
    //
    //     1.3 - COPY: /src and demo_src/ and makes those the guest, host for bundle_fs
    //       respectively
    //
    //     1.4 - save path to pass wo execute in path_cache, use last date after build if unique
    //       names (just a string like '/src/my_app.py'. (this gets appended on to the WORKDIR at
    //       runtime)
    //       do in CMD or ENTRYPOINT or RUN)
    //           - Must check for 'ruby' or 'python'

    println!("Execution complete!");
}

fn execute_label(instr: LabelInstruction) {
  unimplemented!(
        r#"
    handles LABEL
    "#
    )
}

fn execute_env(instr: EnvInstruction) {
  unimplemented!(
        r#"
    handles ENV
    "#
    )
}

fn execute_copy(builder: &mut Builder, instr: CopyInstruction) {
    println!("Building + Bundling runtime, stdlibs, source code, and FS...\n this can take a *several* seconds...\n");
    //builder.bundle_fs("/usr", "./ruby-3.2-wasm32-unknown-wasi-full/usr", "final_build.wasm");
    builder.bundle_fs("/src", "./demo_src", "final_build.wasm");

    println!("Bundling of runtime, source code, and FS is complete!!\n");
    ()
}

fn execute_entrypoint(instr: EntrypointInstruction) {
  unimplemented!(
        r#"
    handles ENTRYPOINT
    "#
    )
}

fn execute_arg(instr: ArgInstruction) {
  unimplemented!(
        r#"
    handles ARG
    "#
    )
}

fn execute_misc(builder: &mut Builder, instr: MiscInstruction) {
    //handles: `MAINTAINER`, `EXPOSE`, `VOLUME`, `Add`
    //`USER`, `WORKDIR`, `ONBUILD`, `STOPSIGNAL`, `HEALTHCHECK`, `SHELL`
    // TODO: start here

    //println!("{:#?}", instr.arguments.components[0] - match on for content);
}


use wasmtime::*;
use std::path::PathBuf;
use anyhow::Result;

fn execute_wasm_with_wasmtime(wasm_file: &str, script: &str) -> Result<()> {
    // todo:
    //let args = vec![PathBuf::from(ruby_script), PathBuf::from(ruby_script)];
    //let args: Vec<String> = args.iter().map(|arg| arg.to_string_lossy().into_owned()).collect();
    //let wasi_ctx =  wasmtime_wasi::WasiCtxBuilder::new().args(&args)?.build();
    //let engine = Engine::default();
    //let mut store = Store::new(&engine, wasi_ctx);
    //let module = Module::from_file(&engine, wasm_file)?;
    //let mut linker = Linker::new(&engine);
    //let instance = linker.instantiate(&mut store, &module)?;
    //let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
    //start.call(&mut store, ())?;


    // shelling out for now, adjust to use as lib
    let command = "wasmtime";
    let args = [wasm_file, "--", script];

    let output = Command::new(command)
        .args(&args)
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command output:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed:\n{}", stderr);

        if let Some(code) = output.status.code() {
            if code != 0 {
                return Err(io::Error::new(ErrorKind::Other, format!("Command exited with status code: {}", code)).into());
            }
        }
    }

    Ok(())
}
