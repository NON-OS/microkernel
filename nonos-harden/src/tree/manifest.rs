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

//! The trust ledger, nonos-data/trust/MANIFEST.sha256, read as untrusted: the
//! form `sha256sum` writes, one `<64 hex>  <path>` (or `<64 hex> *<path>`) per
//! line, every path safe and named once.

use crate::hash::is_sha256_hex;
use crate::release::path::safe;
use std::collections::BTreeSet;

pub const MAX_BYTES: u64 = 64 << 20;
const MAX_LINES: usize = 1_000_000;

pub fn parse(bytes: &[u8]) -> Result<Vec<(String, String)>, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "not UTF-8")?;
    let (mut out, mut seen) = (Vec::new(), BTreeSet::new());
    for (i, line) in text.split_terminator('\n').enumerate() {
        let n = i + 1;
        if n > MAX_LINES {
            return Err("far more lines than a ledger holds".into());
        }
        let sha = line.get(..64).filter(|s| is_sha256_hex(s));
        let sha = sha.ok_or(format!("line {n} does not start with a digest"))?;
        let rel =
            line.get(64..).and_then(|r| r.strip_prefix("  ").or_else(|| r.strip_prefix(" *")));
        let rel = rel.filter(|r| safe(r)).ok_or(format!("line {n} names no safe path"))?;
        if !seen.insert(rel) {
            return Err(format!("line {n} names {rel} a second time"));
        }
        out.push((sha.to_string(), rel.to_string()));
    }
    if out.is_empty() {
        return Err("it names nothing".into());
    }
    Ok(out)
}
