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

//! What a received frame comes to: delivered, answered, or dropped and why.

use alloc::vec::Vec;

use crate::station::Loss;

/// Why a received frame was not delivered.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RxDrop {
    NotAssociated,
    /// Not a data frame with a payload, or shorter than its headers.
    Malformed,
    /// Not FromDS from the BSSID.
    NotFromBss,
    /// Addressed to another station.
    NotForUs,
    Fragmented,
    Amsdu,
    /// An unprotected frame that is not EAPOL on a protected link.
    Unprotected,
    /// No key to decrypt it under, or the MIC failed.
    Undecryptable,
    Replay,
    /// The body is not LLC/SNAP encapsulated.
    NotLlc,
    /// EAPOL arrived with no supplicant to answer it.
    NoSupplicant,
    /// A management frame with nothing for the stack: a beacon, a probe
    /// response, another network's, or one the link does not act on.
    Management,
}

/// What a received frame calls for.
pub enum Rx {
    /// An Ethernet frame for the stack.
    Ethernet(Vec<u8>),
    /// A key handshake message was handled: send `frame` (if any) to the AP,
    /// and install `group_key` (index, key) if the AP delivered a new one.
    Handshake { frame: Option<Vec<u8>>, group_key: Option<(u8, [u8; 16])> },
    /// Send this frame to the AP as it is (an SA Query, protected).
    Send(Vec<u8>),
    /// The link ended; the station is no longer associated.
    Lost(Loss),
    Dropped(RxDrop),
}
