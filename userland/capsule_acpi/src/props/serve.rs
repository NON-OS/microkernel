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

//! PROPERTIES (12, device id u64, offset u32): the device's tree in the
//! wire's form, a page at a time: the whole length u32, then up to
//! `PAGE` bytes from `offset`. Only the device's holder may ask, the
//! kernel's claim test making the call (MkAcpiFunctionOf or
//! MkAcpiDeviceName); the tree is read afresh for each page. EINVAL for a
//! short request or an offset past the end; EIO for a tree too large to
//! send; ENOENT for a device the namespace does not describe.

use super::convert::convert;
use super::wire_write::encode;
use crate::boot::{evaluator, Acpi};
use crate::server::protocol::{EINVAL, EIO, MAX};
use crate::sys::function_of::Asker;
use alloc::vec::Vec;
use nonos_aml::props::tree;

/// What fits a reply after its status and the length word.
pub const PAGE: usize = MAX - 8;

pub fn properties(acpi: &mut Acpi, req: &[u8], sender: u32) -> (i32, Vec<u8>) {
    let (Some(id), Some(at)) = (req.get(1..9), req.get(9..13)) else { return (EINVAL, Vec::new()) };
    let id = u64::from_le_bytes(id.try_into().unwrap_or([0; 8]));
    let at = u32::from_le_bytes(at.try_into().unwrap_or([0; 4])) as usize;
    let dev = match crate::dpower::target(acpi, Asker::Holder(sender), id) {
        Ok(d) => d,
        Err(e) => return (e, Vec::new()),
    };
    let node = tree(&mut acpi.aml, &mut evaluator(&mut acpi.host), dev);
    let Some(wire) = encode(&convert(&acpi.aml, &node)) else { return (EIO, Vec::new()) };
    if at > wire.len() {
        return (EINVAL, Vec::new());
    }
    let mut body = (wire.len() as u32).to_le_bytes().to_vec();
    body.extend_from_slice(&wire[at..wire.len().min(at + PAGE)]);
    (0, body)
}
