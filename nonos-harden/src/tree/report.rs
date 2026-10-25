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

//! The report nonos-verify writes for one module (`<module>-report.json`,
//! nonos-verify/src/report.rs): each check with its status and detail, and
//! the gaps the module says it does not cover. Read defensively all the same.

use crate::release::read::read_regular;
use serde_json::Value;
use std::path::Path;

const MAX_BYTES: u64 = 16 << 20;

pub struct Report {
    pub checks: Vec<Entry>,
    /// Each gap's `what` and `needs`.
    pub gaps: Vec<(String, String)>,
}

pub struct Entry {
    pub name: String,
    pub status: String,
    pub detail: String,
}

pub fn read(path: &Path) -> Result<Report, String> {
    parse(&read_regular(path, MAX_BYTES)?)
}

pub fn parse(bytes: &[u8]) -> Result<Report, String> {
    let v: Value = serde_json::from_slice(bytes).map_err(|e| format!("not JSON ({e})"))?;
    let text = |o: &Value, k: &str| o.get(k).and_then(Value::as_str).map(str::to_string);
    let checks = v.get("checks").and_then(Value::as_array).ok_or("no `checks` list")?;
    let checks = checks
        .iter()
        .map(|c| match (text(c, "name"), text(c, "status")) {
            (Some(name), Some(status)) => {
                Ok(Entry { name, status, detail: text(c, "detail").unwrap_or_default() })
            }
            _ => Err("a check without a name or status".to_string()),
        })
        .collect::<Result<_, _>>()?;
    let gaps = v.get("gaps").and_then(Value::as_array).ok_or("no `gaps` list")?;
    let gaps = gaps
        .iter()
        .map(|g| match (text(g, "what"), text(g, "needs")) {
            (Some(what), Some(needs)) => Ok((what, needs)),
            _ => Err("a gap without `what` or `needs`".to_string()),
        })
        .collect::<Result<_, _>>()?;
    Ok(Report { checks, gaps })
}
