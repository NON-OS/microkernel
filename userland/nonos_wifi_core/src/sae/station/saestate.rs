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
//! hash-to-element, 0 for hunting and pecking), and a commit that only
//! reflects this station's own is dropped. Nothing the AP sends is
//! authenticated until its confirm verifies, so a malformed or refusing
//! frame, or a confirm that does not verify, is discarded (12.4.8.6) with the
//! reason kept for the panel: a forged frame cannot end the exchange, only
//! the driver's deadline.

use super::*;

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
    /// The AP asked for an anti-clogging token more often than `TOKENS_MAX`,
    /// or sent its commit again more often than `CONFIRMS_MAX` allows.
    Looping,
}

/// What a received SAE frame calls for.
pub enum SaeStep {
    /// Send an Authentication frame with this transaction sequence, status
    /// code and SAE body.
    Send { seq: u16, status: u16, body: Vec<u8> },
    /// The AP's confirm verified: the PMK, which exists only from here.
    Accepted { pmk: [u8; 32] },
    /// Nothing to do: a duplicate, a frame for another state, or one
    /// discarded (`SaeStation::failure` says why).
    Ignore,
}
