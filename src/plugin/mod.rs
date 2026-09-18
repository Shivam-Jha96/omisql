use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;
use crate::parser::Statement;

#[derive(Debug, Serialize, Deserialize)]
pub struct LintIssue {
    pub rule_name: String,
    pub severity: String,
    pub message: String,
}

pub struct PluginEngine {
    engine: Engine,
}

impl PluginEngine {
    pub fn new() -> Result<Self> {
        let config = Config::new();
        // Enable WASM features if needed
        let engine = Engine::new(&config)?;
        Ok(Self { engine })
    }

    pub fn run_plugin(&self, plugin_path: &Path, ast: &Statement) -> Result<Vec<LintIssue>> {
        let ast_json = serde_json::to_string(ast)?;
        
        let stdout = wasmtime_wasi::pipe::MemoryOutputPipe::new(1024 * 1024);
        let stdin = wasmtime_wasi::pipe::MemoryInputPipe::new(ast_json.into_bytes());

        let wasi = WasiCtxBuilder::new()
            .stdin(stdin.clone())
            .stdout(stdout.clone())
            .build_p1();

        let mut store = Store::new(&self.engine, wasi);

        let mut linker = Linker::new(&self.engine);
        // wasmtime_wasi v25 has preview1 methods
        wasmtime_wasi::preview1::add_to_linker_sync(&mut linker, |t| t)?;

        let module = Module::from_file(&self.engine, plugin_path)?;
        
        // Link and instantiate
        let instance = linker.instantiate(&mut store, &module)?;
        
        // WASI entry point is usually `_start`
        let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
        
        let _ = start.call(&mut store, ());
        
        let output = stdout.contents();
        let output_str = String::from_utf8(output.to_vec())?;
        
        if output_str.trim().is_empty() {
            return Ok(Vec::new());
        }

        // Output could contain multiple lines or just a JSON array. We assume it's a JSON array.
        match serde_json::from_str::<Vec<LintIssue>>(&output_str) {
            Ok(issues) => Ok(issues),
            Err(e) => {
                // If the plugin didn't return valid JSON, just ignore or log
                println!("Warning: Plugin output was not valid JSON: {}", e);
                println!("Output was: {}", output_str);
                Ok(Vec::new())
            }
        }
    }
}
