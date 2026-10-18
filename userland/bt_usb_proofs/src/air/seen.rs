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

//! What the peer's side sees of the host across links: the address the
//! host connected from, and the identity keys it gave in pairing.

#[derive(Default)]
pub struct Seen {
    /// The address the peer advertises and connects from: PEER unless a
    /// test moves it to a private one.
    pub from: [u8; 6],
    /// The host's random address, once LE Set Random Address set it.
    pub random: Option<[u8; 6]>,
    /// Own_Address_Type of each LE Create Connection, in order.
    pub own_types: Vec<u8>,
    /// Identity Information and Identity Address Information from the
    /// host (Vol 3, Part H, 3.6.4 and 3.6.5), whole PDUs.
    pub identity: Vec<Vec<u8>>,
    /// Whether the host paired on this link, so keys follow encryption.
    pub paired_here: bool,
    /// Pairing Requests the host sent, over all links.
    pub requests: usize,
    /// The address the peer directs its advertising to, if it does:
    /// ADV_DIRECT_IND with this TargetA (Vol 6, Part B, 2.3.1.2).
    pub directed_to: Option<[u8; 6]>,
    /// Scanning_Filter_Policy of each LE Set Scan Parameters, in order.
    pub scan_filters: Vec<u8>,
}
