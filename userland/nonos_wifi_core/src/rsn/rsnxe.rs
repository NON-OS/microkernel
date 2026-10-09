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

//! The RSN Extension element (IEEE Std 802.11-2020, 9.4.2.241). Its first
//! octet carries the field length in bits 0-3 and the "SAE hash-to-element"
//! capability in bit 5. An access point that sets it accepts (or, configured
//! H2E-only, requires) the hash-to-element password element; a station that
//! uses H2E sends the element too, in the association request and again in
//! four-way message 2, and checks the AP's copy in message 3 against the
//! beacon's, as it does the RSNE.

/// The RSNX element id.
pub const EID_RSNX: u8 = 244;
/// The SAE hash-to-element capability bit of the first octet.
pub const RSNX_SAE_H2E: u8 = 1 << 5;

/// The station's RSNXE when it runs SAE hash-to-element: one octet, field
/// length zero (meaning one octet), H2E set.
pub const STATION_RSNXE_H2E: [u8; 3] = [EID_RSNX, 1, RSNX_SAE_H2E];

/// Whether an RSNX element's content (after id and length) advertises SAE
/// hash-to-element. An empty element advertises nothing.
pub fn advertises_h2e(body: &[u8]) -> bool {
    body.first().is_some_and(|b| b & RSNX_SAE_H2E != 0)
}
