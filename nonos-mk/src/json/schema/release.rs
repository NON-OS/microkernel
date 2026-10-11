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

use serde::{Deserialize, Serialize};

use super::commerce::ReleaseExtJson;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseJson {
    pub release_id: String,
    pub manifest_hash: String,
    pub package_hash: String,
    pub package_url: String,
    #[serde(default)]
    pub publisher_signature: String,
    pub supported_arches: Vec<String>,
    pub kernel_abi_min: u32,
    pub required_capabilities: Vec<String>,
    /// Hex of the release's own zk trailer, empty when it ships none.
    #[serde(default)]
    pub zk_trailer_hash: String,
    /// The schema 3 fields. `sign-release` fills in the commerce hash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<ReleaseExtJson>,
    pub validation: ValidationJson,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationJson {
    pub status: String,
    #[serde(default)]
    pub note: String,
    pub validator_id: String,
    pub validated_at_ms: u64,
}
