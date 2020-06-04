/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::custom::WITCustom;

use walrus::ModuleConfig;
use wasmer_wit::{
    decoders::wat::{parse, Buffer},
    encoders::binary::ToBytes,
};

use std::path::PathBuf;

pub struct Config {
    pub in_wasm_path: PathBuf,
    pub out_wasm_path: PathBuf,
    pub wit: String,
}

pub fn embed_wit(options: &Config) -> Result<(), String> {
    let mut module = ModuleConfig::new()
        .parse_file(&options.in_wasm_path)
        .map_err(|e| format!("Failed to parse the Wasm module: {}", e))?;

    let buffer = Buffer::new(&options.wit)
        .map_err(|e| format!("Can't parse provided Wasm module: {}", e))?;
    let ast = parse(&buffer).map_err(|e| format!("Failed to parse the WIT description: {}", e))?;

    let mut bytes = vec![];
    ast.to_bytes(&mut bytes)
        .map_err(|_| "Failed to encode the AST into its binary representation.")?;

    let custom = WITCustom(bytes);
    module.customs.add(custom);
    module
        .emit_wasm_file(&options.out_wasm_path)
        .map_err(|e| format!("Failed to emit Wasm file with bindings: {}", e))?;

    Ok(())
}
