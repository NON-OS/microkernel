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

//! The verify matrix's summary (`summary.json`, tools/nonos-verify-matrix):
//! for a row, each step's command and exit status. A row passes only when it
//! ran steps and every one exited 0; the matrix itself always exits 0, so a
//! non-zero exit means it broke before it judged anything.

use crate::check::{named, Check};
use serde_json::Value;

pub fn steps(bytes: &[u8], row: &str) -> Result<Vec<(String, i64)>, String> {
    let v: Value = serde_json::from_slice(bytes).map_err(|e| format!("not JSON ({e})"))?;
    let steps = v.get("rows").and_then(|r| r.get(row)).and_then(|r| r.get("steps"));
    let steps = steps.and_then(Value::as_array).ok_or(format!("no `rows.{row}.steps` list"))?;
    let step = |s: &Value| match (
        s.get("cmd").and_then(Value::as_str),
        s.get("rc").and_then(Value::as_i64),
    ) {
        (Some(c), Some(rc)) => Ok((c.to_string(), rc)),
        _ => Err("a step without `cmd` or `rc`".to_string()),
    };
    steps.iter().map(step).collect()
}

pub fn judge_row(id: &'static str, row: &str, steps: &[(String, i64)], code: Option<i32>) -> Check {
    if code != Some(0) {
        return Check::fail(
            id,
            format!("the verify matrix exited {code:?} before it judged {row}"),
        );
    }
    if steps.is_empty() {
        return Check::not_run(id, format!("the {row} row ran no steps"));
    }
    let failed: Vec<String> = steps
        .iter()
        .filter(|(_, rc)| *rc != 0)
        .map(|(c, rc)| format!("`{c}` exited {rc}"))
        .collect();
    match failed.is_empty() {
        true => Check::pass(id, format!("all {} steps of the {row} row pass", steps.len())),
        false => Check::fail(id, named(failed)),
    }
}
