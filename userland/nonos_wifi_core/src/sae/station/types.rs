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

//! Where an exchange stands, why a frame was discarded, and what a frame
//! calls for.

use alloc::vec::Vec;

/// Where the exchange stands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SaeState {
    Committed,
    Confirmed,
    Accepted,
}

/// Why a frame from the access point was discarded. Nothing before its
/// confirm verifies is authenticated, so none of these ends the exchange; the
/// last one is kept for the panel if the exchange runs out of time.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SaeFailure {
    /// A frame came with this status code (a wrong password shows only at
    /// the confirm: the AP cannot tell before).
    Rejected(u16),
    /// A frame said the AP does not run group 19.
    GroupNotSupported,
    /// A commit was malformed, used the other PWE method, or gave an
    /// identity shared secret; or an anti-clogging request was malformed.
    BadCommit,
    /// A confirm did not verify: a different password, or a forgery.
    BadConfirm,
}

/// What a received SAE frame calls for.
pub enum SaeStep {
    /// Send an Authentication frame with this transaction sequence, status
    /// code and SAE body.
    Send { seq: u16, status: u16, body: Vec<u8> },
    /// The AP's confirm verified: the PMK is ready.
    Accepted,
    /// Nothing to do: a duplicate, a frame for another state, or one
    /// discarded (`SaeStation::failure` says why).
    Ignore,
}
