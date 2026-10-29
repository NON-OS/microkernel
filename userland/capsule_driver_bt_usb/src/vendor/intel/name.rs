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

//! The file a TLV controller asks for, named as `btintel_get_fw_name_tlv`
//! names it: ibt-<cnvi>-<cnvr>, each the CNVi or CNVr top register's type
//! (bits 11:0) and step (bits 27:24) packed as type << 4 | step and byte
//! swapped, in four hex digits. From variant 0x1e (Blazar) on, a
//! bootloader takes an intermediate loader first (-iml), and the firmware
//! after it is named by the id it reports, when it reports one.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::tlv::{Version, IMG_BOOTLOADER, IMG_OP};

/// The firmware id is the file name's last part; up to its NUL, at most
/// 63 octets of letters, digits and `-_.`, or `None`.
pub fn fw_id(val: &[u8]) -> Option<Vec<u8>> {
    let id = val.split(|&b| b == 0).next().unwrap_or(&[]);
    let named = |b: &u8| b.is_ascii_alphanumeric() || b"-_.".contains(b);
    (id.len() < 64 && id.iter().all(named)).then(|| id.to_vec())
}

pub fn packed(top: u32) -> u16 {
    let (kind, step) = ((top & 0xfff) as u16, ((top >> 24) & 0xf) as u16);
    (kind << 4 | step).swap_bytes()
}

/// The stem of the .sfi (`sfi`) or .ddc file for `v`. A running image is
/// matched against its intermediate loader on Blazar, as Linux compares
/// (`btintel_get_iml_tlv`); DDC always follows the firmware's own name.
pub fn stem(v: &Version, sfi: bool) -> String {
    let base = format!("ibt-{:04x}-{:04x}", packed(v.cnvi_top), packed(v.cnvr_top));
    if v.variant() < 0x1e {
        return base;
    }
    if v.img_type == IMG_BOOTLOADER || (sfi && v.img_type == IMG_OP) {
        return base + "-iml";
    }
    match core::str::from_utf8(&v.fw_id) {
        Ok(id) if !id.is_empty() => base + "-" + id,
        _ => base,
    }
}
