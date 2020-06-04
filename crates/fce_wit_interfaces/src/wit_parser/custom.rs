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

use std::borrow::Cow;
use walrus::{CustomSection, IdsToIndices};

pub const WIT_SECTION_NAME: &str = "interface-types";

#[derive(Debug, Clone)]
pub(crate) struct WITCustom(pub Vec<u8>);

impl CustomSection for WITCustom {
    fn name(&self) -> &str {
        WIT_SECTION_NAME
    }

    fn data(&self, _ids_to_indices: &IdsToIndices) -> Cow<[u8]> {
        Cow::Borrowed(&self.0)
    }
}
