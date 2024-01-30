// This code is based on the wasi-vfs cli, with express permission: https://github.com/kateinoigakukun/wasi-vfs
// Apache License Version 2.0, January 2004

use std::path::PathBuf;
use anyhow::Result;

pub fn pack(wasm_bytes: &[u8], map_dirs: Vec<(PathBuf, PathBuf)>) -> Result<Vec<u8>> {
    std::env::set_var("__WASI_VFS_PACKING", "1");

    let mut wizer = wizer::Wizer::new();
    wizer.allow_wasi(true)?;
    wizer.init_func("wasi_vfs_pack_fs");
    wizer.inherit_stdio(true);
    wizer.inherit_env(true);
    wizer.keep_init_func(true);
    wizer.wasm_bulk_memory(true);
    for (guest_dir, host_dir) in map_dirs {
        wizer.map_dir(guest_dir, host_dir);
    }
    if is_wasi_reactor(wasm_bytes) {
        wizer.func_rename("_initialize", "__wasi_vfs_rt_init");
    }
    let output_bytes = wizer.run(&wasm_bytes)?;
    let output_bytes = copy_export_entry(&output_bytes, "_initialize", "__wasi_vfs_rt_init")?;
    Ok(output_bytes)
}

fn is_wasi_reactor(bytes: &[u8]) -> bool {
    let parser = wasmparser::Parser::new(0);
    for payload in parser.parse_all(bytes) {
        let payload = match payload {
            Ok(payload) => payload,
            Err(_) => continue,
        };
        match payload {
            wasmparser::Payload::ExportSection(export) => {
                for entry in export {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(_) => continue,
                    };
                    if entry.name == "_initialize" {
                        return true;
                    }
                }
                return false;
            }
            wasmparser::Payload::End(_) => return false,
            _ => continue,
        }
    }
    return false;
}

fn copy_export_entry(bytes: &[u8], source: &str, dest: &str) -> Result<Vec<u8>> {
    let mut module = wasm_encoder::Module::new();

    let parser = wasmparser::Parser::new(0);

    for payload in parser.parse_all(bytes) {
        let payload = payload?;
        match payload {
            wasmparser::Payload::Version { .. } => continue,
            wasmparser::Payload::ExportSection(export) => {
                let mut section = wasm_encoder::ExportSection::new();
                for entry in export {
                    let entry = entry?;
                    section.export(entry.name, translate::export_kind(entry.kind), entry.index);
                    if entry.name == source {
                        section.export(dest, translate::export_kind(entry.kind), entry.index);
                    }
                }
                module.section(&section);
            }
            wasmparser::Payload::End(_) => continue,
            _ => {
                if let Some((id, range)) = payload.as_section() {
                    let raw = wasm_encoder::RawSection {
                        id,
                        data: &bytes[range.start..range.end],
                    };
                    module.section(&raw);
                }
            }
        }
    }

    Ok(module.finish())
}

mod translate {
     pub(crate) fn export_kind(x: wasmparser::ExternalKind) -> wasm_encoder::ExportKind {
        match x {
            wasmparser::ExternalKind::Func => wasm_encoder::ExportKind::Func,
            wasmparser::ExternalKind::Table => wasm_encoder::ExportKind::Table,
            wasmparser::ExternalKind::Memory => wasm_encoder::ExportKind::Memory,
            wasmparser::ExternalKind::Global => wasm_encoder::ExportKind::Global,
            wasmparser::ExternalKind::Tag => wasm_encoder::ExportKind::Tag,
        }
    }
}

