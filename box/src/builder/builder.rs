use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::env::current_dir;
use crate::builder::packer;

pub struct Builder {
    base_build: Vec<u8>,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
             base_build: Vec::new(),
        }
    }

    // FROM
    pub fn config_base(&mut self, base: &str) {
        // All builds 'layers' are local for now.
        // Future: move to registry, cache locally
        match base {
            "scratch" => {
                println!("Scratch Build Started...");
            },
            "ruby:3.0" => {
                println!("...Ruby Build Started...\n");
                self.read_file("ruby.wasm");
                println!("Ruby Base Image located \n");
                let length = self.base_build.len();
            },
            "python:3" => {
                println!("...Python Build Started...\n");
                self.read_file("python-3.12.0.wasm");
                println!("Python Base Image located \n");
                let length = self.base_build.len();
            },
            "alpine:latest" => {
            },
            "ubuntu:latest" => {
            },
            "debian:latest" => {
            },
            "centos:latest" => {
            },
            "nginx:alpine" => {
            },
            _ => {
                println!("Base '{}' not recognized.", base);
            }
        }
    }

    // ADD, COPY
    pub fn bundle_fs(&mut self, guest_dir: &str, host_dir: &str, output_file: &str) -> io::Result<()> {
        // generecize for actual vfs + wasi
        let map_dirs = vec![(PathBuf::from(guest_dir), PathBuf::from(host_dir))];

        let output_bytes = packer::pack(&self.base_build, map_dirs)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        println!("hi");

        let output_path = PathBuf::from(output_file);
        //println!("{:#?}", output_path.display());
        fs::write(output_path, output_bytes)?;

        Ok(())
    }

    // COMMAND, RUN, ENTRYPOINT
    pub fn expose_exec_interfaces(&self) {
      unimplemented!()
    }

    pub fn build(&self, wasm_only: bool) {
        // Makes single executable: runtime, .wasm, args, OR just wasm + args
        // If single executable, make the host run exec command on execution.

        // future: bindings to drop in a js , or a node env, etc., could
        // interface with signatures.
    }

    fn read_file(&mut self, filename: &str) -> io::Result<()> {
        let mut path = PathBuf::from("./base_builds");
        path.push(filename);

        let mut file = fs::File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        self.base_build = contents;
        Ok(())
    }
}
