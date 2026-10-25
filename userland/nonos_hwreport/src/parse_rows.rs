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

//! The PCI and ACPI lines' fields.

use crate::acpi::hashed;
use crate::parse_fields::{dec, four, hex};

/// `vvvv:dddd class=cc.ss.pp claimed=0|1 external=0|1`
pub(crate) fn pci_ok(w: &[u8]) -> bool {
    let flag = |f: &[u8], key: &[u8]| f.strip_prefix(key).is_some_and(|v| v == b"0" || v == b"1");
    let Some(f) = four(w) else { return false };
    let id = f[0].len() == 9 && f[0][4] == b':' && hex(&f[0][..4], 4) && hex(&f[0][5..], 4);
    let class = f[1].strip_prefix(b"class=").is_some_and(|c| {
        c.len() == 8
            && c[2] == b'.'
            && c[5] == b'.'
            && [0, 3, 6].iter().all(|&o| hex(&c[o..o + 2], 2))
    });
    id && class && flag(f[2], b"claimed=") && flag(f[3], b"external=")
}

/// `SIG instance length digest|-`, the digest present exactly when hashed.
pub(crate) fn acpi_ok(w: &[u8]) -> bool {
    let Some(f) = four(w) else { return false };
    let Ok(sig) = <[u8; 4]>::try_from(f[0]) else { return false };
    let sig_ok = sig.iter().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || *b == b'_');
    let digest = if hashed(&sig) { hex(f[3], 64) } else { f[3] == b"-" };
    sig_ok
        && dec(f[1]).is_some_and(|v| v <= u64::from(u32::MAX))
        && dec(f[2]).is_some_and(|v| v <= u64::from(u32::MAX))
        && digest
}
