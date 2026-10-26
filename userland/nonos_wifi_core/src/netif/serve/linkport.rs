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

//! Answer net_core's link operations for a WiFi interface. The four operations
//! are the whole contract: report the link state and the station MAC, take
//! Ethernet frames to transmit, and hand back received Ethernet frames. All the
//! WiFi specifics (association, CCMP, the firmware rings) live behind the
//! `LinkPort` so this translation is exact and testable without a radio.

use super::*;

/// The WiFi link as net_core needs to see it: an Ethernet-framed interface.
/// Implementations translate to and from 802.11 behind these methods.
pub trait LinkPort {
    /// The station MAC address once associated, else None.
    fn mac(&self) -> Option<[u8; 6]>;
    /// Whether the station is associated and carrying data.
    fn link_up(&self) -> bool;
    /// Copy the next received Ethernet frame into `out`, returning its length,
    /// or None when nothing is queued.
    fn poll_rx(&mut self, out: &mut [u8]) -> Option<usize>;
    /// Transmit one Ethernet frame; returns false if it could not be sent
    /// (for instance while unassociated).
    fn send_tx(&mut self, frame: &[u8]) -> bool;
}

/// Handle one net_core request, writing the response into `out` and returning
/// its length. Returns None only for a malformed request or an undersized
/// buffer; every known operation produces a well-formed response, including the
/// "no frame" and "link down" cases net_core expects.
pub fn serve<L: LinkPort>(req: &[u8], link: &mut L, out: &mut [u8]) -> Option<usize> {
    let r = parse_request(req)?;
    match r.op {
        OP_LINK_STATUS => status_byte(out, OP_LINK_STATUS, r.request_id, 0, link.link_up() as u8),
        OP_MAC_ADDRESS => mac_response(out, r.request_id, link.mac()),
        OP_TX_PACKET => {
            let status = if link.send_tx(r.payload) { 0 } else { -STATUS_NO_LINK };
            status_only(out, OP_TX_PACKET, r.request_id, status)
        }
        OP_RX_PACKET => rx_response(out, (r.request_id, r.payload), link),
        _ => None,
    }
}
