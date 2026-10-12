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
//! What LE Create Connection asks for (Vol 4, Part E, 7.8.12): the
//! controller scans for the one named device and connects when it hears
//! it advertise.

use alloc::vec::Vec;

use super::scan::Scan;
use crate::error::HciError;
use nonos_bt_wire::addr::{AddrType, BdAddr};
use nonos_bt_wire::l2cap::ConnParams;
use nonos_bt_wire::WireError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Connect {
    pub scan: Scan,
    pub peer_type: AddrType,
    pub peer: BdAddr,
    pub link: ConnParams,
}

/// 20 to 40 ms between connection events, none skipped, and a 4 s
/// supervision timeout: a key press waits at most one interval, and a
/// keyboard that goes out of range is noticed within seconds.
pub const DEFAULT_LINK: ConnParams =
    ConnParams { interval_min: 0x0010, interval_max: 0x0020, latency: 0, timeout: 0x0190 };

/// Initiator_Filter_Policy 0: connect to the named peer, not to whoever
/// is on the Filter Accept List.
const NAMED_PEER: u8 = 0x00;
/// The 25 parameter octets with Own_Address_Type public, or `Reserved`
/// for timing outside the ranges 7.8.12 allows (the connection ranges are
/// those `ConnParams` checks).
pub fn create_connection_params(c: &Connect) -> Result<Vec<u8>, HciError> {
    create_connection_params_as(c, AddrType::Public)
}

/// As above, as `own`: random once a random address is set (7.8.4).
pub fn create_connection_params_as(c: &Connect, own: AddrType) -> Result<Vec<u8>, HciError> {
    if !c.scan.valid() || !c.link.acceptable() {
        return Err(HciError::Wire(WireError::Reserved));
    }
    let mut p = Vec::with_capacity(25);
    p.extend_from_slice(&c.scan.interval.to_le_bytes());
    p.extend_from_slice(&c.scan.window.to_le_bytes());
    p.extend_from_slice(&[NAMED_PEER, c.peer_type.wire()]);
    p.extend_from_slice(&c.peer.0);
    p.push(own.wire());
    let l = c.link;
    for v in [l.interval_min, l.interval_max, l.latency, l.timeout] {
        p.extend_from_slice(&v.to_le_bytes());
    }
    // Minimum and maximum connection event length: 0 leaves them to the
    // controller; they are only a hint (Vol 4, Part E, 7.8.12).
    p.extend_from_slice(&[0; 4]);
    Ok(p)
}
