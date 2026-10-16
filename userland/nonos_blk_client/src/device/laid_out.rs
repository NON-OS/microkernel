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

//! An identify reply in the layout the SATA driver set and the SD reader
//! follows (`identity_parse.rs`), or `None` from a driver built before it
//! had one: the disk is still listed, by its bus and size alone.

use super::identity_parse::{parse_ahci_identity, Identity, AHCI_IDENTIFY_LEN};
use crate::driver::Driver;
use crate::wire::{call, decode_reply, HDR_LEN, STATUS_LEN};

pub(super) fn laid_out(driver: Driver, op: u16, port: u32) -> Option<Identity> {
    let magic = driver.magic();
    let mut rx = [0u8; HDR_LEN + STATUS_LEN + AHCI_IDENTIFY_LEN];
    let (n, id) = call(port, magic, op, &[], &mut rx).ok()?;
    let body = decode_reply(&rx, n, magic, op, id).ok()?;
    parse_ahci_identity(body)
}
