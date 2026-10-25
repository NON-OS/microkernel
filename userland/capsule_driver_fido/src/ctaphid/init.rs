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
//! A channel of our own (CTAPHID_INIT, as recalled, unverified): asked on
//! the broadcast channel with an 8-byte nonce, answered with the nonce, the
//! new channel, the protocol version, the key's version and its
//! capabilities. The nonce only tells our answer from another's; this
//! driver is the key's one host, so uptime and a counter suffice.

use super::frame::{Cid, BROADCAST};

/// Capability bits: WINK, CBOR, and NMSG (no CTAP1 messages).
pub const CAP_CBOR: u8 = 0x04;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Channel {
    pub cid: Cid,
    pub protocol: u8,
    /// Major, minor and build of the key's own version.
    pub version: [u8; 3],
    pub caps: u8,
}

/// The channel INIT's answer `data` gives for `nonce`, if it is ours.
pub fn channel(data: &[u8], nonce: [u8; 8]) -> Option<Channel> {
    if data.len() < 17 || data[..8] != nonce {
        return None;
    }
    let cid = [data[8], data[9], data[10], data[11]];
    if cid == BROADCAST || cid == [0; 4] {
        return None;
    }
    let version = [data[13], data[14], data[15]];
    Some(Channel { cid, protocol: data[12], version, caps: data[16] })
}
