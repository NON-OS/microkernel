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

//! Two fields of the build receipt, each read as untrusted: an artifact the
//! seal measured, and the commit id it was built from.

use crate::hash::is_sha256_hex;
use serde_json::Value;

/// The receipt names an artifact by the path the seal saw it at; only the
/// file name is shown, quoted safely.
pub fn artifact(path: &str, a: &Value) -> Result<(String, Option<String>), String> {
    let name = path.rsplit('/').next().unwrap_or(path).escape_debug().to_string();
    if a.is_null() {
        return Ok((name, None));
    }
    let sha = a.get("sha256").and_then(Value::as_str).filter(|s| is_sha256_hex(s));
    let sha = sha.ok_or(format!("artifact {name} has no proper `sha256`"))?;
    Ok((name, Some(sha.to_string())))
}

pub fn is_commit(c: &str) -> bool {
    matches!(c.len(), 40 | 64)
        && c.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
