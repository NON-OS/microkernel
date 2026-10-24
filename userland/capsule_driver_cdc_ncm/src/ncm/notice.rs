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
//! What NETWORK_CONNECTION on the notification endpoint says (ECM 1.2,
//! 6.3.1, as recalled, unverified): an 8-byte header shaped like a setup
//! packet, bmRequestType A1h, bNotification 00h, wValue 1 when the link
//! is up and 0 when it is down.

const FROM_INTERFACE_CLASS: u8 = 0xA1;
const NETWORK_CONNECTION: u8 = 0x00;

/// The link state a notification `n` gives, or `None` for another.
pub fn connected(n: &[u8]) -> Option<bool> {
    let h = n.get(..8)?;
    (h[0] == FROM_INTERFACE_CLASS && h[1] == NETWORK_CONNECTION).then(|| h[2] != 0 || h[3] != 0)
}
