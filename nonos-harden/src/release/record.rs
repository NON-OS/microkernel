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

//! The release record the seal writes (format `nonos-release/1`), read as
//! untrusted input: any shape it does not expect is refused with a reason,
//! never a panic, and no path it names leaves the release.

use super::config::config;
use super::entry::entry;
use serde_json::Value;

pub const FORMAT: &str = "nonos-release/1";
/// A record is a few kilobytes; anything far larger is not one.
pub const MAX_BYTES: u64 = 16 << 20;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub release: bool,
    pub profile: String,
    pub features: Vec<String>,
    pub rollback_index: Option<u32>,
    pub sealed: Vec<Sealed>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sealed {
    pub name: String,
    pub rel: String,
    pub item: Item,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    /// One file and its SHA-256.
    File(String),
    /// A directory: every file in it, by path inside it, and its SHA-256.
    Dir(Vec<(String, String)>),
}

pub fn parse(bytes: &[u8]) -> Result<Record, String> {
    if bytes.len() as u64 > MAX_BYTES {
        return Err(format!("{} bytes, far more than a record holds", bytes.len()));
    }
    let v: Value = serde_json::from_slice(bytes).map_err(|e| format!("not JSON ({e})"))?;
    let top = v.as_object().ok_or("not a JSON object")?;
    match top.get("format").and_then(Value::as_str) {
        Some(FORMAT) => {}
        Some(f) => return Err(format!("format `{}`, not `{FORMAT}`", f.escape_debug())),
        None => return Err("no `format` string".into()),
    }
    let cfg = top.get("config").and_then(Value::as_object).ok_or("no `config` object")?;
    let (release, profile, features, rollback_index) = config(cfg)?;
    let sealed = top.get("sealed").and_then(Value::as_object).ok_or("no `sealed` object")?;
    if sealed.is_empty() {
        return Err("`sealed` names nothing".into());
    }
    let sealed = sealed.iter().map(|(k, v)| entry(k, v)).collect::<Result<_, _>>()?;
    Ok(Record { release, profile, features, rollback_index, sealed })
}
