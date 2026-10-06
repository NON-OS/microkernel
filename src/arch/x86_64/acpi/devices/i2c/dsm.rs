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

//! The HID descriptor register from `_DSM`. HID over I2C v1.0 section 8.2
//! (and Linux i2c-hid-acpi) asks the device's `_DSM` with GUID
//! 3CDFF6F7-4267-4555-AD05-B30A3D8938DE, revision 1, function 1. Firmware
//! writes it almost always as
//!
//!   If (Arg0 == ToUUID ("3cdff6f7-...")) {
//!       If (Arg2 == Zero) { ... Return (Buffer () { 0x03 }) }
//!       If (Arg2 == One) { Return (0x20) }
//!   }
//!
//! so the answer is the constant (or the constant initial value of a named
//! integer) returned under `Arg2 == One` inside the branch that compares
//! Arg0 with that GUID. Anything computed at run time reads as unknown, and
//! the drivers probe the registers in use (0x0001 ELAN, 0x0020 Synaptics).

use super::names::{integer, is_name_seg, method_body, named_integer};
use crate::arch::x86_64::acpi::aml::scan::read_pkg_length;

/// The GUID as ToUUID lays it out in AML.
pub(super) const HID_I2C_DSM_GUID: [u8; 16] = [
    0xF7, 0xF6, 0xDF, 0x3C, 0x67, 0x42, 0x55, 0x45, 0xAD, 0x05, 0xB3, 0x0A, 0x3D, 0x89, 0x38, 0xDE,
];

const IF_OP: u8 = 0xA0;
const LEQUAL_OP: u8 = 0x93;
const RETURN_OP: u8 = 0xA4;
const ARG0: u8 = 0x68;
const ARG2: u8 = 0x6A;
const ONE_OP: u8 = 0x01;
/// How far past `Arg2 == One` the Return may sit: the If's PkgLength and the
/// predicate are a handful of bytes.
const RETURN_WINDOW: usize = 12;

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

/// The If package that compares Arg0 with the GUID found at `guid_at`, or
/// everything from the GUID on when that If cannot be delimited.
fn guid_branch(terms: &[u8], guid_at: usize) -> &[u8] {
    for back in 1..=12usize {
        let Some(p) = guid_at.checked_sub(back) else { break };
        if terms[p] != IF_OP {
            continue;
        }
        if let Some((len, lb)) = read_pkg_length(terms, p + 1) {
            let pred = p + 1 + lb;
            if terms.get(pred) == Some(&LEQUAL_OP) && terms.get(pred + 1) == Some(&ARG0) {
                return &terms[p..(p + 1 + len).min(terms.len())];
            }
        }
    }
    &terms[guid_at..]
}

/// The value returned for function 1 inside `branch`, resolving a returned
/// name against `device_body`.
fn function_one(branch: &[u8], device_body: &[u8]) -> Option<u16> {
    let mut from = 0usize;
    while let Some(rel) = branch.get(from..).and_then(|b| {
        b.windows(3).position(|w| {
            w[0] == LEQUAL_OP && ((w[1] == ARG2 && w[2] == ONE_OP) || (w[1] == ONE_OP && w[2] == ARG2))
        })
    }) {
        let at = from + rel + 3;
        let window = &branch[at..(at + RETURN_WINDOW).min(branch.len())];
        if let Some(r) = window.iter().position(|&b| b == RETURN_OP) {
            let v = &branch[at + r + 1..];
            let value = match integer(v) {
                Some((n, _)) => Some(n),
                None => v.get(..4).filter(|s| is_name_seg(s)).and_then(|seg| {
                    let mut name = [0u8; 4];
                    name.copy_from_slice(seg);
                    named_integer(device_body, &name)
                }),
            };
            // Zero is the initial value of a name the firmware fills in at
            // run time far more often than it is a real register.
            return value.filter(|&n| n != 0 && n <= u64::from(u16::MAX)).map(|n| n as u16);
        }
        from = at;
    }
    None
}

/// The HID descriptor register the device's `_DSM` declares, when static.
pub(super) fn hid_descriptor_register(device_body: &[u8]) -> Option<u16> {
    let terms = method_body(device_body, b"_DSM")?;
    if let Some(guid_at) = find(terms, &HID_I2C_DSM_GUID) {
        return function_one(guid_branch(terms, guid_at), device_body);
    }
    // The GUID can live in a named buffer the method compares Arg0 against.
    // Trust the method body only when the device declares that GUID and the
    // method holds no other UUID literal that could own the branch found.
    let other_uuid_literal = find(terms, &[0x11, 0x13, 0x0A, 0x10]).is_some();
    if find(device_body, &HID_I2C_DSM_GUID).is_some() && !other_uuid_literal {
        return function_one(terms, device_body);
    }
    None
}
