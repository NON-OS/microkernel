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

//! Hex for the schema 3 fields, where empty means zero or nothing.

use crate::error::CliError;

pub(crate) fn hex_bytes(s: &str, field: &'static str) -> Result<Vec<u8>, CliError> {
    let trimmed = s.strip_prefix("0x").unwrap_or(s);
    hex::decode(trimmed).map_err(|e| CliError::Json(format!("{field}: {e}")))
}

pub(crate) fn hex_n<const N: usize>(s: &str, field: &'static str) -> Result<[u8; N], CliError> {
    if s.is_empty() {
        return Ok([0u8; N]);
    }
    let bytes = hex_bytes(s, field)?;
    bytes.as_slice().try_into().map_err(|_| CliError::BadHexLen {
        field,
        expected: N,
        got: bytes.len(),
    })
}

pub(crate) fn hex_or_empty(bytes: &[u8]) -> String {
    if bytes.iter().all(|&b| b == 0) {
        String::new()
    } else {
        hex::encode(bytes)
    }
}

pub(crate) fn decimal(s: &str, field: &'static str) -> Result<u128, CliError> {
    if s.is_empty() {
        return Ok(0);
    }
    s.parse::<u128>().map_err(|_| CliError::Json(format!("{field}: not a u128: {s}")))
}
