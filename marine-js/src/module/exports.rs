/*
 * Copyright 2022 Fluence Labs Limited
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

use super::IValue;
use super::IType;
use super::IFunctionArg;

use wasmer_it::interpreter::wasm;

// In current implementation export simply does nothing, because there is no more
// explicit instruction call-export in this version of wasmer-interface-types,
// but explicit Exports is still required by wasmer-interface-types::Interpreter.
#[derive(Clone)]
pub(crate) struct ITExport {
    name: String,
    arguments: Vec<IFunctionArg>,
    outputs: Vec<IType>,
    function: fn(arguments: &[IValue]) -> Result<Vec<IValue>, ()>,
}

impl ITExport {
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        Self {
            name: String::new(),
            arguments: vec![],
            outputs: vec![],
            function: |_| -> _ { Ok(vec![]) },
        }
    }
}

impl wasm::structures::Export for ITExport {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn inputs_cardinality(&self) -> usize {
        self.arguments.len()
    }

    fn outputs_cardinality(&self) -> usize {
        self.outputs.len()
    }

    fn arguments(&self) -> &[IFunctionArg] {
        &self.arguments
    }

    fn outputs(&self) -> &[IType] {
        &self.outputs
    }

    fn call(&self, arguments: &[IValue]) -> Result<Vec<IValue>, ()> {
        (self.function)(arguments)
    }
}
