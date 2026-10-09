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

//! Every `Device (...)` in an AML block with the name of the object that
//! encloses it. Firmware commonly declares a touchpad as
//! `Scope (\_SB.PCI0.I2C1) { Device (TPD0) { ... } }` and leaves the
//! I2cSerialBus ResourceSource to be filled at run time, so the enclosing
//! scope is the only static record of which controller the pad hangs off.
//!
//! A bounded byte scan, like the rest of the extractor: a Scope or Device
//! opcode only counts when its package length fits inside the enclosing
//! object and its name is a well-formed AML NameString, which keeps a stray
//! 0x10 inside a buffer from opening a phantom scope.

use alloc::vec::Vec;

use super::names::is_name_seg;
use crate::arch::x86_64::acpi::aml::scan::{read_pkg_length, skip_name_string};

const EXT_OP_PREFIX: u8 = 0x5B;
const DEVICE_OP: u8 = 0x82;
const SCOPE_OP: u8 = 0x10;
const MAX_STEPS: usize = 4_000_000;
const MAX_DEPTH: usize = 32;

pub(super) struct DeviceNode<'a> {
    /// The Device's own NameSeg.
    pub name: [u8; 4],
    /// The NameSeg of the Scope or Device that encloses it, zero at the root.
    pub parent: [u8; 4],
    /// The Device's own objects: its package up to the first nested Device.
    pub body: &'a [u8],
}

/// Validate a NameString spanning `bytes`: optional root or parent prefixes,
/// then one NameSeg, a dual or a multi name path, every segment well formed.
fn valid_name_string(bytes: &[u8]) -> bool {
    let mut i = 0;
    while i < bytes.len() && (bytes[i] == b'\\' || bytes[i] == b'^') {
        i += 1;
    }
    let segs = match bytes.get(i) {
        Some(0x2E) => &bytes[i + 1..],
        Some(0x2F) => bytes.get(i + 2..).unwrap_or(&[]),
        Some(_) => &bytes[i..],
        None => return false,
    };
    !segs.is_empty() && segs.len() % 4 == 0 && segs.chunks(4).all(is_name_seg)
}

fn first_nested_device(body: &[u8]) -> usize {
    body.windows(2).position(|w| w[0] == EXT_OP_PREFIX && w[1] == DEVICE_OP).unwrap_or(body.len())
}

pub(super) fn devices(aml: &[u8]) -> Vec<DeviceNode<'_>> {
    let mut out = Vec::new();
    // Open scopes: (end offset, NameSeg).
    let mut stack: Vec<(usize, [u8; 4])> = Vec::new();
    let mut i = 0usize;
    let mut steps = 0usize;
    while i + 1 < aml.len() && steps < MAX_STEPS {
        steps += 1;
        while stack.last().is_some_and(|s| i >= s.0) {
            stack.pop();
        }
        let op_len = if aml[i] == EXT_OP_PREFIX && aml[i + 1] == DEVICE_OP {
            2
        } else if aml[i] == SCOPE_OP {
            1
        } else {
            i += 1;
            continue;
        };
        let is_device = op_len == 2;
        let pkg_at = i + op_len;
        let enclosing_end = stack.last().map_or(aml.len(), |s| s.0);
        let parsed = read_pkg_length(aml, pkg_at).and_then(|(pkg_len, len_bytes)| {
            let pkg_end = pkg_at.checked_add(pkg_len)?;
            let name_at = pkg_at + len_bytes;
            let name_end = skip_name_string(aml, name_at)?;
            let fits = pkg_end <= enclosing_end && name_end <= pkg_end;
            (fits && valid_name_string(&aml[name_at..name_end])).then_some((pkg_end, name_end))
        });
        let Some((pkg_end, name_end)) = parsed else {
            i += 1;
            continue;
        };
        let mut name = [0u8; 4];
        name.copy_from_slice(&aml[name_end - 4..name_end]);
        if is_device {
            let body = &aml[name_end..pkg_end];
            let own = &body[..first_nested_device(body)];
            let parent = stack.last().map_or([0; 4], |s| s.1);
            out.push(DeviceNode { name, parent, body: own });
        }
        if stack.len() < MAX_DEPTH {
            stack.push((pkg_end, name));
        }
        i = name_end;
    }
    out
}
