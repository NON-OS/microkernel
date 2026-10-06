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

//! REMOTE_NDIS_INITIALIZE_MSG and its completion (Remote NDIS 1.0, 2.2.1
//! and 2.2.2), as Linux generic_rndis_bind sends and reads them.

use nonos_usbnet::nic::ETH_FRAME_MAX;

use super::message::{le32, put32, MEDIUM_802_3, MSG_INIT};
use super::packet::PACKET_HDR;

/// What the device said it takes in one bulk OUT transfer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Limits {
    pub max_packets: u32,
    pub max_transfer: u32,
    /// Messages after the first in a transfer start on 1 << this. One
    /// message goes per transfer here, so it never applies to what is sent.
    pub align_shift: u32,
}

/// RNDIS 1.0 and the most the host takes in one bulk IN, BULK_MAX at most.
pub fn init_msg(host_max: u32) -> [u8; 24] {
    let mut m = [0u8; 24];
    for (at, v) in [(0, MSG_INIT), (4, 24), (12, 1), (16, 0), (20, host_max)] {
        put32(&mut m, at, v);
    }
    m
}

/// The device's limits from an INITIALIZE_CMPLT of `r.len()` bytes (its
/// MessageLength), or the reason it cannot carry this driver's frames.
pub fn init_done(r: &[u8]) -> Result<Limits, &'static str> {
    let word = |at| le32(r, at).ok_or("INITIALIZE_CMPLT short");
    if word(28)? != MEDIUM_802_3 {
        return Err("medium not 802.3");
    }
    let (max_packets, max_transfer, align_shift) = (word(32)?, word(36)?, word(40)?);
    // A full frame, its header and a padding byte in one transfer. Linux
    // lowers the MTU for a smaller device; the stack here sends 1514.
    if (max_transfer as usize) < PACKET_HDR + ETH_FRAME_MAX + 1 {
        return Err("device transfer size under one frame");
    }
    Ok(Limits { max_packets: max_packets.max(1), max_transfer, align_shift })
}
