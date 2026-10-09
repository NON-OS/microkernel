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

use super::MSS;

/// What a peer that sent no MSS option takes (RFC 9293 3.7.1: 576 - 40).
const DEFAULT_MSS: u16 = 536;

/// The floor for an announced MSS, so a peer cannot have every byte sent in
/// its own segment. Linux uses the same figure.
const MIN_MSS: u16 = 88;

/*
 * The largest segment to send this peer, from the MSS its SYN announced.
 * Segments larger than that leave with Don't Fragment set and are dropped
 * at the narrow link (PPPoE, a tunnel) with nothing coming back that this
 * stack reads, so the connection stalls.
 */
pub fn send_mss(announced: Option<u16>) -> u16 {
    announced.unwrap_or(DEFAULT_MSS).clamp(MIN_MSS, MSS as u16)
}
