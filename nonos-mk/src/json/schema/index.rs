// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Serde shape mirrors the field names in
//! `abi/marketplace_index.schema.json`. Hex-encoded fields stay as
//! strings here and are converted at the IR boundary; numeric and
//! enum fields keep the JSON spelling so a hand-edited JSON file is
//! the authority.

use serde::{Deserialize, Serialize};

use super::entry::EntryJson;

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexJson {
    pub schema_version: u32,
    pub operator_id: String,
    pub published_at_ms: u64,
    pub serial: u64,
    pub entries: Vec<EntryJson>,
}
