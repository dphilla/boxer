use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::process::Command;
use dockerfile_parser::*;

mod builder {
    pub mod builder;
    pub mod packer;
}

use builder::builder::*;

fn main() {
    let dockerfile = Dockerfile::parse(r#"
       FROM python:3
       WORKDIR /usr/src/app
       COPY . .
       CMD [ "python", "./your-script.py" ]
    "#).unwrap();


    //FROM ruby:3.0
    //WORKDIR /usr/src/app
    //COPY . .
    //CMD ["./your-script.rb"]

    let mut builder = Builder::new();
    for stage in dockerfile.iter_stages() {
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
            Instruction::Misc(instr) => execute_misc(instr.clone()),
        }
      }

      builder.build(true);
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
    // 1) python build
    // 2) box build and box run
    // 3) make box command
    // 4) Web builder demo - build + run, download-able link for executable
    //
    let wasm_file = "final_build.wasm";
    //let script = "demo_src/my_app.rb";
    let script = "src/my_app.py";

    if let Err(e) = execute_wasm_with_wasmtime(wasm_file, script) {
        eprintln!("Error executing WASM with Wasmtime: {}", e);
    }

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
    println!("Building + Bundling ruby, source code, and FS...\n this can take a *several* seconds...\n");
    //builder.bundle_fs("/usr", "./ruby-3.2-wasm32-unknown-wasi-full/usr", "final_build.wasm");
    builder.bundle_fs("/src", "./demo_src", "final_build.wasm");

    println!("Bundling Ruby, source code, and FS is complete!!\n");
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

fn execute_misc(instr: MiscInstruction) {
  //unimplemented!(
        //r#"
    //handles: `MAINTAINER`, `EXPOSE`, `VOLUME`, `Add`
    //`USER`, `WORKDIR`, `ONBUILD`, `STOPSIGNAL`, `HEALTHCHECK`, `SHELL`
    //"#
    //)
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
