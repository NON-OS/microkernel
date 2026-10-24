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

//! The kernel whose signature verifies is the kernel the release boots. The
//! bytes verified must be the bytes the record seals as `kernel`, and the
//! ESP (`esp`) must carry exactly one `kernel.bin`, those same bytes: the
//! loader reads the kernel from the ESP, so a signed kernel beside an
//! unsigned one on the ESP proves nothing about what boots.

use super::record::{Item, Record};
use crate::hash::{sha256_hex, short};

/// The digest the record seals the kernel file under, once the bytes read
/// for the signature check are shown to be those bytes and the ESP's.
pub fn bound(rec: &Record, want: &str, bytes: &[u8]) -> Result<(), String> {
    let got = sha256_hex(bytes);
    if got != want {
        let (g, w) = (short(&got), short(want));
        return Err(format!("the bytes verified (sha256 {g}…) are not the sealed {w}…"));
    }
    let esp = rec.sealed.iter().find(|s| s.name == "esp");
    let Some(Item::Dir(files)) = esp.map(|s| &s.item) else {
        return Err("the record seals no ESP directory to boot the kernel from".into());
    };
    let named = |p: &str| p == "kernel.bin" || p.ends_with("/kernel.bin");
    let on_esp: Vec<&(String, String)> = files.iter().filter(|(p, _)| named(p)).collect();
    match on_esp.as_slice() {
        [(_, sha)] if sha == want => Ok(()),
        [(p, sha)] => {
            let (s, w) = (short(sha), short(want));
            Err(format!("the ESP's {p} (sha256 {s}…) is not the signed kernel {w}…"))
        }
        [] => Err("the ESP carries no kernel.bin".into()),
        _ => Err("the ESP carries more than one kernel.bin".into()),
    }
}
