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

//! Finding a named object in a device body and reading a constant out of it.
//! Bounded byte scans over AML, never an interpreter: a value the firmware
//! computes at run time reads as `None`.

use crate::arch::x86_64::acpi::aml::scan::{read_pkg_length, skip_name_string};

const NAME_OP: u8 = 0x08;
const METHOD_OP: u8 = 0x14;
const BUFFER_OP: u8 = 0x11;
const MAX_STEPS: usize = 1_000_000;

/// The bytes from the value of `Name (target, value)` in `body` to its end.
pub(super) fn name_value<'a>(body: &'a [u8], target: &[u8; 4]) -> Option<&'a [u8]> {
    let mut i = 0usize;
    while i < body.len() && i < MAX_STEPS {
        if body[i] == NAME_OP {
            if let Some(end) = skip_name_string(body, i + 1) {
                if end >= i + 5 && &body[end - 4..end] == target {
                    return body.get(end..);
                }
            }
        }
        i += 1;
    }
    None
}

/// The term list of `Method (target, ...)` in `body`, bounded by the method's
/// package length.
pub(super) fn method_body<'a>(body: &'a [u8], target: &[u8; 4]) -> Option<&'a [u8]> {
    let mut i = 0usize;
    while i < body.len() && i < MAX_STEPS {
        if body[i] == METHOD_OP {
            if let Some((pkg_len, len_bytes)) = read_pkg_length(body, i + 1) {
                let pkg_end = (i + 1).saturating_add(pkg_len).min(body.len());
                if let Some(end) = skip_name_string(body, i + 1 + len_bytes) {
                    if end >= 4 && end < pkg_end && &body[end - 4..end] == target {
                        // One MethodFlags byte, then the term list.
                        return body.get(end + 1..pkg_end);
                    }
                }
            }
        }
        i += 1;
    }
    None
}

/// A `_CRS`-style value: a `Name` holding a Buffer, bounded to that buffer,
/// or the body of a `Method` of that name.
pub(super) fn buffer_or_method<'a>(body: &'a [u8], target: &[u8; 4]) -> Option<&'a [u8]> {
    if let Some(v) = name_value(body, target) {
        if v.first() == Some(&BUFFER_OP) {
            if let Some((pkg_len, _)) = read_pkg_length(v, 1) {
                return v.get(..(1 + pkg_len).min(v.len()));
            }
        }
        return Some(v);
    }
    method_body(body, target)
}

/// Decode an AML integer constant at the start of `v`: Zero, One, Ones, or a
/// Byte/Word/DWord/QWord prefixed constant. Returns the value and its length.
pub(super) fn integer(v: &[u8]) -> Option<(u64, usize)> {
    let le = |n: usize| -> Option<u64> {
        let bytes = v.get(1..1 + n)?;
        Some(bytes.iter().rev().fold(0u64, |acc, &b| (acc << 8) | u64::from(b)))
    };
    match *v.first()? {
        0x00 => Some((0, 1)),
        0x01 => Some((1, 1)),
        0xFF => Some((u64::MAX, 1)),
        0x0A => Some((le(1)?, 2)),
        0x0B => Some((le(2)?, 3)),
        0x0C => Some((le(4)?, 5)),
        0x0E => Some((le(8)?, 9)),
        _ => None,
    }
}

/// `Name (target, <integer>)` in `body`. A `Method` of that name is not
/// evaluated: its first `Return` may sit in one branch of several.
pub(super) fn named_integer(body: &[u8], target: &[u8; 4]) -> Option<u64> {
    integer(name_value(body, target)?).map(|(v, _)| v)
}

/// True when `seg` is a valid AML NameSeg: a leading letter or underscore,
/// then letters, digits or underscores.
pub(super) fn is_name_seg(seg: &[u8]) -> bool {
    seg.len() == 4
        && (seg[0].is_ascii_uppercase() || seg[0] == b'_')
        && seg[1..].iter().all(|&c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == b'_')
}
