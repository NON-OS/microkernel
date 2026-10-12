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

//! The firmware's missed-beacons notification. Its second word counts the
//! access point's beacons missed since anything was received from it; at
//! Linux iwlwifi's threshold the link asks after the access point at once
//! rather than wait out its own quiet.

/// MISSED_BEACONS_NOTIF, in the MAC configuration group.
pub(super) const MISSED_BEACONS_NOTIF: u8 = 0xF6;
/// IWL_MVM_MISSED_BEACONS_SINCE_RX_THOLD.
const MISSED_SINCE_RX: u32 = 4;

/// Whether a missed-beacons notification says the access point went quiet.
pub(super) fn quiet(payload: &[u8]) -> bool {
    payload
        .get(4..8)
        .is_some_and(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]) >= MISSED_SINCE_RX)
}
