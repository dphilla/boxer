use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::collections::HashMap;

use wasmtime::{Engine, Module, Store, Linker, Memory};

/// This must match the struct on the Wasm side in `system.rs`:
/// ```
/// #[repr(C)]
/// pub struct FileDef {
///     pub dest_path: *const i8,
///     pub data_ptr: *const u8,
///     pub data_len: u32,
/// }
/// ```
#[repr(C)]
pub struct FileDef {
    pub path_off: u32,
    pub data_off: u32,
    pub data_len: u32,
}

pub struct Builder {
    pub base_build: Vec<u8>,
    pub working_directory: PathBuf,
    pub copied_files: Vec<(String, Vec<u8>)>,  // (dest_path, file_data)
}

impl Builder {
    pub fn new() -> Self {
        Self {
            base_build: Vec::new(),
            working_directory: PathBuf::from("/"),
            copied_files: Vec::new(),
        }
    }

    /// Called from `execute_from` in main.rs
    pub fn config_base(&mut self, base: &str) {
        match base {
            "scratch" => {
                println!("Scratch Build Started...");
                match fs::read("output_wizered.wasm") {
                    Ok(bytes) => {
                        self.base_build = bytes;
                    }
                    Err(e) => {
                        println!("No 'output_wizered.wasm' found, or error reading file: {:?}", e);
                        println!("Falling back to empty scratch environment...");
                        self.base_build = Vec::new();
                    }
                }
            },
            _ => {
                println!("Base '{}' not recognized. Doing nothing.", base);
            }
        }
    }

    /// Called from `execute_copy` in main.rs
    /// to add `(dest_path, data_bytes)` pairs.
    /// We'll ephemeral-run them in `build()`.
    pub fn bundle_fs_from_buffer(&mut self, buffer: HashMap<String, Vec<u8>>) {
        println!("Bundling files into the Wasm virtual filesystem...");
        for (path, content) in buffer {
            println!("COPY -> path: {}", path);
            self.copied_files.push((path, content));
        }
    }

    /// The final ephemeral run that calls `wasm_vfs_mount_in_memory`.
    pub fn build(&mut self, _wasm_only: bool) {
        if self.base_build.is_empty() {
            println!("No base .wasm to finalize. Possibly scratch environment is empty.");
            return;
        }
        if self.copied_files.is_empty() {
            println!("No files to copy; skipping mount_in_memory step.");
            return;
        }

        // 1) Create an Engine & compile the Wasm
        let engine = Engine::default();
        let module = match Module::new(&engine, &self.base_build) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error compiling base_build: {:?}", e);
                return;
            }
        };

        // 2) Create a Linker & Store. If your module needs imports, define them here:
        let mut linker = Linker::new(&engine);
        let mut store = Store::new(&engine, ());

        // 3) Instantiate
        let instance = match linker.instantiate(&mut store, &module) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("Error instantiating module: {:?}", e);
                return;
            }
        };

        // 4) Get the exported memory
        let memory = match instance.get_memory(&mut store, "memory") {
            Some(mem) => mem,
            None => {
                eprintln!("No exported memory named 'memory'.");
                return;
            }
        };

        // 5) We'll place each file in guest memory + build an array of FileDef
        let size_of_fd = std::mem::size_of::<FileDef>();
        if size_of_fd != 12 {
            eprintln!("Warning: FileDef is not 12 bytes? Adjust if needed.");
        }

        let count = self.copied_files.len();
        let total_filedef_bytes = count * size_of_fd;

        // We'll pick an offset for these arrays
        let mut guest_cursor = 0x10000; // e.g. 64k
        let filedef_array_base = guest_cursor;
        guest_cursor += total_filedef_bytes;

        // Helper for copying data into memory
        let mut copy_to_guest = |host_data: &[u8], offset: usize| {
            if let Err(e) = memory.write(&mut store, offset, host_data) {
                eprintln!("Failed to write to guest memory: {:?}", e);
            }
        };

        // Build the host side buffer that holds all FileDef structs
        let mut filedef_structs = vec![0u8; total_filedef_bytes];

        let mut idx = 0;
        for (dest_path, data) in &self.copied_files {
            // convert path to null-terminated
            let mut path_bytes = dest_path.as_bytes().to_vec();
            path_bytes.push(0);

            // copy path into guest memory
            let path_offset = guest_cursor;
            copy_to_guest(&path_bytes, path_offset);
            guest_cursor += path_bytes.len();

            // copy data
            let data_offset = guest_cursor;
            copy_to_guest(data, data_offset);
            guest_cursor += data.len();

            // fill in fields for FileDef
            // offset into filedef_structs:
            let base = idx * size_of_fd;
            idx += 1;

            // path_off
            filedef_structs[base..base+4].copy_from_slice(&(path_offset as u32).to_le_bytes());
            // data_off
            filedef_structs[base+4..base+8].copy_from_slice(&(data_offset as u32).to_le_bytes());
            // data_len
            filedef_structs[base+8..base+12].copy_from_slice(&(data.len() as u32).to_le_bytes());
        }

        // copy FileDef array to guest
        copy_to_guest(&filedef_structs, filedef_array_base);

        // 6) call wasm_vfs_mount_in_memory(count, array_ptr)
        let mount_func = match instance.get_func(&mut store, "wasm_vfs_mount_in_memory") {
            Some(f) => f,
            None => {
                eprintln!("No export named wasm_vfs_mount_in_memory");
                return;
            }
        };

        let typed_mount = match mount_func.typed::<(u32, u32), i32>(&store) {
            Ok(tf) => tf,
            Err(e) => {
                eprintln!("Signature mismatch for wasm_vfs_mount_in_memory: {:?}", e);
                return;
            }
        };

        let count_u32 = self.copied_files.len() as u32;
        let ptr = filedef_array_base as u32;

        match typed_mount.call(&mut store, (count_u32, ptr)) {
            Ok(retval) => {
                println!("wasm_vfs_mount_in_memory returned: {}", retval);
            }
            Err(e) => {
                eprintln!("Error calling mount_in_memory: {:?}", e);
            }
        }

        println!("Ephemeral run done; the in-memory FS has your copied files.");
    }
}

