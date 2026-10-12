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

//! The two Realtek commands this setup sends, both in the vendor-specific
//! group (OGF 0x3F, Vol 4, Part E, 5.4.1) and answered by Command
//! Complete like any other: Read ROM Version, whose one return octet picks
//! the patch, and Download, which carries the patch in fragments.

use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_wire::hci::opcode::Opcode;
use nonos_bt_wire::WireError;

pub const READ_ROM_VERSION: Opcode = Opcode::new(0x3f, 0x06d);
pub const DOWNLOAD: Opcode = Opcode::new(0x3f, 0x020);

/// The ROM version: its one return octet after the status.
pub fn rom_version<T: Transport>(host: &mut Host<T>) -> Result<u8, HciError> {
    match host.command(READ_ROM_VERSION, &[])?.as_slice() {
        [v] => Ok(*v),
        _ => Err(HciError::Wire(WireError::Length)),
    }
}

/// One fragment sent: its index octet, then its data. The controller
/// answers with the index it took, one octet after the status.
pub fn download<T: Transport>(host: &mut Host<T>, index: u8, data: &[u8]) -> Result<(), HciError> {
    let mut params = alloc::vec::Vec::with_capacity(1 + data.len());
    params.push(index);
    params.extend_from_slice(data);
    match host.command(DOWNLOAD, &params)?.as_slice() {
        [_] => Ok(()),
        _ => Err(HciError::Wire(WireError::Length)),
    }
}
