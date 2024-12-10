use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::env::current_dir;
use crate::builder::packer;

pub struct Builder {
    base_build: Vec<u8>,
    pub working_directory: PathBuf,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
             base_build: Vec::new(),
             working_directory: PathBuf::from("/")
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
            // etc.
            _ => {
                println!("Base '{}' not recognized.", base);
            }
        }
    }

    // COPY
    pub fn bundle_fs_from_buffer(&mut self, buffer: HashMap<String, Vec<u8>>) {
        println!("Bundling files into the Wasm virtual filesystem...");
        for (path, content) in buffer {
            println!("Adding file to FS: {}", path);
            // TODO: add wasm-vfs, map buffer accordingly
            // Example: self.wasm_vfs.add_file(path, content);
        }
    }

    // COMMAND, RUN, ENTRYPOINT
    pub fn expose_exec_interfaces(&self) {
      unimplemented!()
    }

    pub fn build(&self, wasm_only: bool) {
    }

    pub fn set_working_directory(&mut self, dir: PathBuf) {
        self.working_directory = dir;
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
