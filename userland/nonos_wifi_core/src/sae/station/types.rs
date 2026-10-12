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

//! Where an exchange stands, why one failed, and what a frame calls for.

use alloc::vec::Vec;

/// Where the exchange stands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SaeState {
    Committed,
    Confirmed,
    Accepted,
    Failed,
}

/// Why an exchange ended without a PMK.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SaeFailure {
    /// The AP answered with this status code (a wrong password ends here
    /// only at the confirm: the AP cannot tell before).
    Rejected(u16),
    /// The AP does not run group 19.
    GroupNotSupported,
    /// The AP's commit was malformed, used the other PWE method, or gave an
    /// identity shared secret.
    BadCommit,
    /// The AP's confirm did not verify: a different password.
    BadConfirm,
}

/// What a received SAE frame calls for.
pub enum SaeStep {
    /// Send an Authentication frame with this transaction sequence, status
    /// code and SAE body.
    Send { seq: u16, status: u16, body: Vec<u8> },
    /// The AP's confirm verified: the PMK is ready.
    Accepted,
    /// Nothing to do (a duplicate, or a frame for another state).
    Ignore,
    /// The exchange failed; see `SaeStation::failure`.
    Failed,
}
