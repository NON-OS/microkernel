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

//! What one cell means while a link is being brought up.

use crate::cell::{CELL_AUTH_CHALLENGE, CELL_CERTS, CELL_NETINFO, CELL_VPADDING};

/// The fixed-size PADDING command; a relay may send it at any time.
const CELL_PADDING: u8 = 0;

/// What the client does with a cell before its NETINFO is sent.
#[derive(Debug, PartialEq, Eq)]
pub enum Next {
    /// Verify the relay's certificates against its consensus identity.
    Certs,
    /// Nothing to do: padding, or an offer only a relay would answer.
    Ignore,
    /// The relay's NETINFO after its identity is proved: answer and finish.
    Finish,
    /// Anything else ends the handshake.
    Refuse,
}

/// Classify one cell. CERTS, AUTH_CHALLENGE and VPADDING are variable-length
/// cells; NETINFO and PADDING are fixed-length (tor-spec section 3), so the
/// size of the frame is part of what makes a cell what it claims to be.
pub fn classify(variable: bool, command: u8, bound: bool) -> Next {
    match (variable, command) {
        (true, CELL_CERTS) => Next::Certs,
        (true, CELL_AUTH_CHALLENGE) | (true, CELL_VPADDING) | (false, CELL_PADDING) => Next::Ignore,
        // NETINFO before a verified CERTS would finish the handshake with a
        // peer that never proved who it is, so it is refused rather than
        // accepted and checked afterwards.
        (false, CELL_NETINFO) if bound => Next::Finish,
        _ => Next::Refuse,
    }
}
