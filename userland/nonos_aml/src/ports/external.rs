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

//! PCIe root ports that face the outside of the machine, as Microsoft's
//! Kernel DMA Protection documentation has firmware declare them in _DSD:
//! "ExternalFacingPort" (1 on a hot-plug port a user can reach, such as a
//! Thunderbolt or USB4 port) under one UUID, and "DmaProperty" under
//! another. Both UUIDs and both names are recalled, not read from the
//! document, which could not be reached here: unverified.

use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::Eval;
use crate::serial::{properties_under, Value};

/// EFCC06CC-73AC-4BC3-BFF0-76143807C389, in ToUUID's byte order.
pub const EXTERNAL_FACING_UUID: [u8; 16] = [
    0xCC, 0x06, 0xCC, 0xEF, 0xAC, 0x73, 0xC3, 0x4B, 0xBF, 0xF0, 0x76, 0x14, 0x38, 0x07, 0xC3, 0x89,
];
/// 70D24161-6DD5-4C9E-8070-705531292865, in ToUUID's byte order.
pub const DMA_PROPERTY_UUID: [u8; 16] = [
    0x61, 0x41, 0xD2, 0x70, 0xD5, 0x6D, 0x9E, 0x4C, 0x80, 0x70, 0x70, 0x55, 0x31, 0x29, 0x28, 0x65,
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Facing {
    pub external: bool,
    /// DmaProperty's value, None when the port does not declare it.
    pub dma_property: Option<u64>,
}

fn integer_named(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    port: Id,
    uuid: &[u8; 16],
    name: &[u8],
) -> Option<u64> {
    let props = properties_under(aml, eval, port, uuid);
    props.into_iter().find(|p| p.name == name).and_then(|p| match p.value {
        Value::Integer(v) => Some(v),
        _ => None,
    })
}

pub fn facing(aml: &mut Aml, eval: &mut Eval<'_>, port: Id) -> Facing {
    let external = integer_named(aml, eval, port, &EXTERNAL_FACING_UUID, b"ExternalFacingPort");
    let dma_property = integer_named(aml, eval, port, &DMA_PROPERTY_UUID, b"DmaProperty");
    Facing { external: external.is_some_and(|v| v != 0), dma_property }
}
