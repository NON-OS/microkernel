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

//! The accepted releases, one a line:
//!
//! ```text
//! release VERSION PCR8 PCR9 LOADER FRONT NETWORK,...|-
//! ```

use super::files::read;
use crate::connect::args::hex;
use alloc::string::String;
use alloc::vec::Vec;
use nsh_attest::Release;

pub fn releases(path: &str) -> Result<Vec<Release>, String> {
    let text = String::from_utf8(read(path)?).map_err(|_| alloc::format!("{path}: not text"))?;
    let lines = text.lines().map(|l| l.split('#').next().unwrap_or("").trim());
    lines
        .filter(|l| !l.is_empty())
        .map(|l| release(l).ok_or_else(|| alloc::format!("{path}: {l}")))
        .collect()
}

fn release(line: &str) -> Option<Release> {
    let f: Vec<&str> = line.split_whitespace().collect();
    let ["release", version, p8, p9, loader, front, network] = f[..] else { return None };
    let network = match network {
        "-" => Vec::new(),
        n => n.split(',').map(hex::<32>).collect::<Option<Vec<_>>>()?,
    };
    let (pcr8, pcr9, loader, front) = (hex(p8)?, hex(p9)?, hex(loader)?, hex(front)?);
    Some(Release { version: version.into(), pcr8, pcr9, loader, front, network })
}
