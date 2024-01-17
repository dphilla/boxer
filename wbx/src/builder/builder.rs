pub struct Builder {
    // stateful repr of buildsteps
}

impl Builder {
    pub fn new() -> Self {
        Builder {
        }
    }

    // FROM
    pub fn config_base(&self, base: &str) {
        match base {
            "scratch" => {
                println!("Scratch Build Started...", base);
            },
            "ruby:3.0" => {
                // check for local wasm build, else pull
            },
            "python:3" => {
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
    pub fn bundle_fs(&self) {
    }

    // COMMAND, RUN, ENTRYPOINT
    pub fn expose_exec_interfaces(&self) {
    }

    pub fn build(&self, wasm_only: bool) {
        // makes single executable: runtime, .wasm, args, OR just wasm + args
    }
}
