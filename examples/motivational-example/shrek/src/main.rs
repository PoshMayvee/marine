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

#![allow(improper_ctypes)]

use marine_rs_sdk::marine;

fn main() {}

#[marine]
pub fn greeting(name: &str) -> Vec<String> {
    let donkey_greeting = donkey::greeting(name);
    let shrek_greeting = format!("Shrek: hi, {}", name);

    vec![shrek_greeting, donkey_greeting]
}

mod donkey {
    use super::*;

    #[marine]
    #[link(wasm_import_module = "donkey")]
    extern "C" {
        pub fn greeting(name: &str) -> String;
    }
}
