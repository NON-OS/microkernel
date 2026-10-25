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

//! What the receive half hands back for a data frame.

use super::*;

/// What a received frame calls for.
pub enum Rx {
    /// An Ethernet frame for the stack.
    Ethernet(Vec<u8>),
    /// A key handshake message was handled: send `frame` (if any) to the AP,
    /// install `group_key` (index, key) if the AP delivered a new CCMP-128
    /// one, and `pairwise_key` in place of the pairwise key once `frame` has
    /// gone out under the old one (a rekey). A GCMP-256 key stays with the
    /// station.
    Handshake {
        frame: Option<Vec<u8>>,
        group_key: Option<(u8, [u8; 16])>,
        pairwise_key: Option<[u8; 16]>,
    },
    /// Send this frame to the AP as it is (an SA Query, protected).
    Send(Vec<u8>),
    /// The link ended; the station is no longer associated.
    Lost(Loss),
    Dropped(RxDrop),
}
