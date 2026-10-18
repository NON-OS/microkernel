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

//! HID over I2C (Microsoft's HID over I2C protocol specification): a
//! device whose _HID or _CID is PNP0C50 or ACPI0C50, whose HID descriptor
//! sits at the register its _DSM function 1 returns. Firmware may compute
//! it at run time, which is why it is asked and never assumed.

use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, Eval, HardwareId};
use alloc::vec;

/// 3CDFF6F7-4267-4555-AD05-B30A3D8938DE, in ToUUID's byte order.
pub const HID_I2C_DSM_UUID: [u8; 16] = [
    0xF7, 0xF6, 0xDF, 0x3C, 0x67, 0x42, 0x55, 0x45, 0xAD, 0x05, 0xB3, 0x0A, 0x3D, 0x89, 0x38, 0xDE,
];
const REVISION: u64 = 1;
const DESCRIPTOR_ADDRESS: u64 = 1;

pub fn is_hid_over_i2c(ids: &[HardwareId]) -> bool {
    ids.iter().any(|h| matches!(h.bytes(), b"PNP0C50" | b"ACPI0C50"))
}

/// _DSM(UUID, 1, 1, {}): the HID descriptor register. None when there is
/// no _DSM, it fails, or it answers anything but an integer that fits.
pub fn descriptor_register(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Option<u16> {
    let args = vec![
        Object::buffer(HID_I2C_DSM_UUID.to_vec()),
        Object::Integer(REVISION),
        Object::Integer(DESCRIPTOR_ADDRESS),
        Object::package(vec![]),
    ];
    match child(aml, eval, dev, *b"_DSM", args)?.ok()? {
        Object::Integer(v) => u16::try_from(v).ok(),
        _ => None,
    }
}
