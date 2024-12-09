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
            //
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
    use std::fs;
    use std::io::ErrorKind;
    use std::path::Path;
    use walkdir::WalkDir;
    use std::collections::HashMap;

    // In-memory structure to store files and their destinations
    let mut wasm_fs_buffer: HashMap<String, Vec<u8>> = HashMap::new();

    for src in &instr.sources {
        let src_path = Path::new(src);
        let dest_path = Path::new(&instr.destination);

        if !src_path.exists() {
            eprintln!("Error: Source path {} does not exist.", src_path.display());
            continue;
        }

        if src_path.is_file() {
            match fs::read(src_path) {
                Ok(file_content) => {
                    let dest_file_path = dest_path
                        .join(src_path.file_name().unwrap())
                        .to_string_lossy()
                        .to_string();
                    wasm_fs_buffer.insert(dest_file_path, file_content);
                }
                Err(e) => eprintln!("Error reading file {}: {}", src_path.display(), e),
            }
        } else if src_path.is_dir() {
            for entry in WalkDir::new(src_path) {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_file() {
                            match fs::read(path) {
                                Ok(file_content) => {
                                    let relative_path = path.strip_prefix(src_path).unwrap();
                                    let dest_file_path = dest_path
                                        .join(relative_path)
                                        .to_string_lossy()
                                        .to_string();
                                    wasm_fs_buffer.insert(dest_file_path, file_content);
                                }
                                Err(e) => eprintln!("Error reading file {}: {}", path.display(), e),
                            }
                        }
                    }
                    Err(e) => eprintln!("Error traversing directory {}: {}", src_path.display(), e),
                }
            }
        } else {
            eprintln!("Error: Unsupported path type for {}.", src_path.display());
        }
    }

    builder.bundle_fs_from_buffer(wasm_fs_buffer);
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
