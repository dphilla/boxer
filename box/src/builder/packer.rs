use anyhow::Result;
use std::path::PathBuf;

pub fn pack(wasm_bytes: &[u8], _map_dirs: Vec<(PathBuf, PathBuf)>) -> Result<Vec<u8>> {
    // Check if `_initialize` is present as an exported function
    let has_initialize = is_wasi_reactor(wasm_bytes);

    // If `_initialize` is present, copy it as `_start`.
    let output_bytes = if has_initialize {
        copy_export_entry(wasm_bytes, "_initialize", "_start")?
    } else {
        wasm_bytes.to_vec()
    };

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
    false
}

fn copy_export_entry(bytes: &[u8], source: &str, dest: &str) -> Result<Vec<u8>> {
    let mut module = wasm_encoder::Module::new();
    let parser = wasmparser::Parser::new(0);

    for payload in parser.parse_all(bytes) {
        let payload = payload?;
        match payload {
            wasmparser::Payload::Version { .. } => {
                // wasm_encoder::Module will handle this automatically
                continue;
            }
            wasmparser::Payload::ExportSection(export) => {
                let mut section = wasm_encoder::ExportSection::new();
                for entry in export {
                    let entry = entry?;
                    section.export(entry.name, translate::export_kind(entry.kind), entry.index);
                    if entry.name == source {
                        // Duplicate the source export under the new name
                        section.export(dest, translate::export_kind(entry.kind), entry.index);
                    }
                }
                module.section(&section);
            }
            wasmparser::Payload::End(_) => {
                // End of module - handled automatically by wasm_encoder
                continue;
            }
            _ => {
                // For all other sections, just copy them through
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

