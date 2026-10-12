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

//! The station side of one SAE exchange with an access point (IEEE Std
//! 802.11-2020, 12.4.8): send a commit; answer an anti-clogging request by
//! sending it again with the token; on the AP's commit derive the keys and
//! send a confirm; on the AP's confirm check it and accept. The status code of
//! the AP's commit must say the same PWE method this station used (126 for
//! hash-to-element, 0 for hunting and pecking), a commit that only reflects
//! this station's own is dropped, and anything malformed or refused ends the
//! exchange with the reason kept for the panel.

use super::*;

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
