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

//! The gaps a nonos-verify module may declare while the verdict is still GO,
//! as the checked tree lists them (scripts/baselines/declared-limits.txt,
//! held to the source and to shrinking by scripts/check_declared_limits.py).
//! A list that is missing or not in its format is refused, so no gap can pass.

use crate::release::read::read_regular;
use std::path::Path;

pub const FILE: &str = "scripts/baselines/declared-limits.txt";
const HEADER: &str = "# format: nonos-harden-declared-limits/1";
const MAX_BYTES: u64 = 1 << 20;

/// Each line's module and gap, `<module>: <what>`.
pub type Declared = Vec<(String, String)>;

pub fn load(tree: &Path) -> Result<Declared, String> {
    let bytes = read_regular(&tree.join(FILE), MAX_BYTES);
    bytes.and_then(|b| parse(&b)).map_err(|e| format!("{FILE}: {e}"))
}

pub fn parse(bytes: &[u8]) -> Result<Declared, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "not UTF-8")?;
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err(format!("its first line is not `{HEADER}`"));
    }
    let mut out = Vec::new();
    for (i, line) in lines.enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        match line.split_once(": ") {
            Some((m, w)) if !m.is_empty() && !w.trim().is_empty() => out.push((m.into(), w.into())),
            _ => return Err(format!("line {} is not `<module>: <what>`", i + 2)),
        }
    }
    Ok(out)
}
