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

#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod wasm_mem_pages_conversion;

pub use wasm_mem_pages_conversion::*;

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct SharedString(pub Rc<String>);

impl std::borrow::Borrow<str> for SharedString {
    fn borrow(&self) -> &str {
        self.0.as_str()
    }
}
