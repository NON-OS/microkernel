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

//! The PORT_LIST and IDENTIFY replies.

use crate::emmc::info::MEDIUM_EMMC;
use crate::emmc::{Opened, SECTOR_SIZE};
use crate::protocol::{
    encode_identify, Request, IDENTIFY_PAYLOAD_LEN, PORT_ENTRY_BYTES, PORT_LIST_HEADER_BYTES,
};

use super::reply::body_reply;

/// One entry for the single slot.
pub(super) fn port_list(o: &mut Opened, req: &Request, tx: &mut [u8]) {
    let entry = o.disk.port_entry();
    let mut out = [0u8; PORT_LIST_HEADER_BYTES + PORT_ENTRY_BYTES];
    out[..4].copy_from_slice(&1u32.to_le_bytes());
    out[PORT_LIST_HEADER_BYTES..].copy_from_slice(&entry[..PORT_ENTRY_BYTES]);
    body_reply(tx, req, &out);
}

/// The part's capacity, model and serial, with the medium set to eMMC.
pub(super) fn identify(o: &mut Opened, req: &Request, tx: &mut [u8]) {
    let names = o.disk.names();
    let mut out = [0u8; IDENTIFY_PAYLOAD_LEN];
    encode_identify(
        &mut out,
        o.disk.capacity_sectors(),
        SECTOR_SIZE as u32,
        names.model(),
        &names.serial,
        MEDIUM_EMMC,
    );
    body_reply(tx, req, &out);
}
