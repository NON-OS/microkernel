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

//! The USB serial number (iSerialNumber), which tells one stick from another
//! of the same make and size; only its digest leaves here (`digest`).

use super::digest::digest;
use super::types::Disk;
use crate::xhci::control_in;

const GET_DESCRIPTOR: (u8, u8) = (0x80, 0x06);
const DEVICE: u16 = 0x0100;
const STRING: u16 = 0x0300;
/// iSerialNumber's offset in the 18-byte device descriptor.
const I_SERIAL: usize = 16;
/// US English, asked when the device lists no language.
const EN_US: u16 = 0x0409;

/// The digest of the serial number's characters, or None when the device
/// names none or a request for it fails. A serial number is optional, and a
/// stick whose string did not read was refused; it is bound with none, and
/// served no more after a let-go (scan/release.rs).
pub fn serial(disk: &Disk) -> Option<u64> {
    let mut device = [0u8; 18];
    control_in(disk.xhci, disk.slot, GET_DESCRIPTOR, DEVICE, 0, &mut device).ok()?;
    let index = device[I_SERIAL];
    if index == 0 {
        return None;
    }
    let mut langs = [0u8; 4];
    let lang = match string(disk, 0, 0, &mut langs) {
        Some(4) => u16::from_le_bytes([langs[2], langs[3]]),
        _ => EN_US,
    };
    let mut named = [0u8; 255];
    let n = string(disk, index, lang, &mut named)?;
    named.get(2..n).filter(|chars| !chars.is_empty()).map(digest)
}

/// String descriptor `index` in `lang`: how many of the bytes that came its
/// own length covers, or None when the request failed.
fn string(disk: &Disk, index: u8, lang: u16, out: &mut [u8]) -> Option<usize> {
    let ask =
        control_in(disk.xhci, disk.slot, GET_DESCRIPTOR, STRING | u16::from(index), lang, out);
    Some(usize::from(out.first().copied().unwrap_or(0)).min(ask.ok()?))
}
