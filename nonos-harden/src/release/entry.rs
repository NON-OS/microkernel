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

//! One `sealed` entry of the record: `{"file": rel, "sha256": hex}` for a
//! file, `{"dir": rel, "files": {path: hex, ...}}` for a directory. Exactly
//! one of the two shapes, every path safe, every digest 64 hex digits.

use super::path::{safe, safe_name};
use super::record::{Item, Sealed};
use crate::hash::is_sha256_hex;
use serde_json::Value;

/// More files than any ESP holds; a record naming more is refused.
const MAX_FILES: usize = 100_000;

pub fn entry(name: &str, v: &Value) -> Result<Sealed, String> {
    if !safe_name(name) {
        return Err(format!("sealed entry `{}` has an unusable name", name.escape_debug()));
    }
    let bad = |what: &str| format!("sealed `{name}`: {what}");
    let o = v.as_object().ok_or_else(|| bad("not an object"))?;
    let rel = |key: &str| o.get(key).map(|p| p.as_str().filter(|p| safe(p)));
    match (rel("file"), rel("dir")) {
        (Some(Some(f)), None) => {
            let sha = o.get("sha256").and_then(Value::as_str).filter(|s| is_sha256_hex(s));
            let sha = sha.ok_or_else(|| bad("`sha256` is not 64 lowercase hex digits"))?;
            Ok(Sealed { name: name.into(), rel: f.into(), item: Item::File(sha.into()) })
        }
        (None, Some(Some(d))) => {
            let files = o.get("files").and_then(Value::as_object);
            let files = files.ok_or_else(|| bad("`files` is not an object"))?;
            if files.len() > MAX_FILES {
                return Err(bad("names more files than any sealed directory holds"));
            }
            let mut out = Vec::with_capacity(files.len());
            for (p, sha) in files {
                let sha = sha.as_str().filter(|s| is_sha256_hex(s));
                match (safe(p), sha) {
                    (true, Some(sha)) => out.push((p.clone(), sha.to_string())),
                    _ => return Err(bad("a file in it has an unsafe path or a bad digest")),
                }
            }
            Ok(Sealed { name: name.into(), rel: d.into(), item: Item::Dir(out) })
        }
        (Some(_), Some(_)) => Err(bad("names both a file and a directory")),
        _ => Err(bad("has no safe relative `file` or `dir` path")),
    }
}
