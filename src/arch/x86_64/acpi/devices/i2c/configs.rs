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

//! Which ACPI devices are HID over I2C. The specification (and Linux
//! i2c-hid-acpi) identifies them by the compatible id PNP0C50 (or ACPI0C50),
//! which firmware usually places in `_CID` under a vendor `_HID` such as
//! SYNA3602 or ELAN0718. Intel reference firmware even ships `_HID`
//! "XXXX0000" and patches it at run time, so `_CID` is the only stable
//! signal. A vendor `_HID` prefix without the compatible id is still taken as
//! a candidate; the bus probe decides whether it speaks HID over I2C.

use alloc::vec::Vec;

use super::names::{method_body, name_value};
use crate::arch::x86_64::acpi::aml::scan::read_pkg_length;

const DWORD_PREFIX: u8 = 0x0C;
const STRING_PREFIX: u8 = 0x0D;
const PACKAGE_OP: u8 = 0x12;
const RETURN_OP: u8 = 0xA4;
const MAX_IDS: usize = 8;

/// Expand a compressed EISA id (as AML stores it) into seven characters plus
/// a zero byte.
fn expand_eisaid(b: [u8; 4]) -> [u8; 8] {
    let hex = |n: u8| if n < 10 { b'0' + n } else { b'A' + n - 10 };
    [
        ((b[0] >> 2) & 0x1F) + b'@',
        (((b[0] & 0x03) << 3) | (b[1] >> 5)) + b'@',
        (b[1] & 0x1F) + b'@',
        hex(b[2] >> 4),
        hex(b[2] & 0x0F),
        hex(b[3] >> 4),
        hex(b[3] & 0x0F),
        0,
    ]
}

/// Decode one id object (EISAID DWord or String) at the start of `v`,
/// returning it and the bytes it occupies.
fn decode_one(v: &[u8]) -> Option<([u8; 8], usize)> {
    match *v.first()? {
        DWORD_PREFIX => {
            let r = v.get(1..5)?;
            Some((expand_eisaid([r[0], r[1], r[2], r[3]]), 5))
        }
        STRING_PREFIX => {
            let len = v[1..].iter().position(|&b| b == 0)?;
            if len == 0 {
                return None;
            }
            let mut out = [0u8; 8];
            let n = len.min(8);
            out[..n].copy_from_slice(&v[1..1 + n]);
            Some((out, len + 2))
        }
        _ => None,
    }
}

/// Every id an id object at `v` holds: one EISAID or String, or a Package of
/// them (the `_CID` form for several compatible ids).
pub(super) fn decode_ids(v: &[u8]) -> Vec<[u8; 8]> {
    let mut out = Vec::new();
    if v.first() == Some(&PACKAGE_OP) {
        let Some((pkg_len, len_bytes)) = read_pkg_length(v, 1) else { return out };
        let end = (1 + pkg_len).min(v.len());
        let count = v.get(1 + len_bytes).copied().unwrap_or(0) as usize;
        let mut at = 2 + len_bytes;
        for _ in 0..count.min(MAX_IDS) {
            let Some((id, used)) = v.get(at..end).and_then(decode_one) else { break };
            out.push(id);
            at += used;
        }
    } else if let Some((id, _)) = decode_one(v) {
        out.push(id);
    }
    out
}

/// The ids a device declares under `target` (`_HID` or `_CID`): a `Name`, or
/// a `Method` whose first `Return` is a constant id.
pub(super) fn ids_of(body: &[u8], target: &[u8; 4]) -> Vec<[u8; 8]> {
    if let Some(v) = name_value(body, target) {
        return decode_ids(v);
    }
    if let Some(terms) = method_body(body, target) {
        if let Some(at) = terms.iter().position(|&b| b == RETURN_OP) {
            return decode_ids(&terms[at + 1..]);
        }
    }
    Vec::new()
}

/// PNP0C50 (seven characters) or ACPI0C50: the HID-over-I2C compatible id.
pub(super) fn is_hid_over_i2c_id(id: &[u8; 8]) -> bool {
    (&id[..7] == b"PNP0C50" && id[7] == 0) || id == b"ACPI0C50"
}

/// A `_HID` from a vendor that ships HID-over-I2C touchpads.
pub(super) fn has_touchpad_vendor_prefix(hid: &[u8; 8]) -> bool {
    const PREFIXES: [&[u8]; 6] = [b"ELAN", b"SYNA", b"ALPS", b"FTE", b"CYAP", b"MSFT"];
    PREFIXES.iter().any(|p| hid.starts_with(p))
}
