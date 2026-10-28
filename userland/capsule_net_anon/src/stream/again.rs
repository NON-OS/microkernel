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

//! Which ENDs before CONNECTED send a stream's open once more, to another
//! exit.

use super::end::{REASON_EXITPOLICY, REASON_HIBERNATING, REASON_NOROUTE};
use super::held::Held;

pub const REASON_TIMEOUT: u8 = 7;
pub const REASON_RESOURCELIMIT: u8 = 11;

/*
 * The exit's policy, its route out, its hibernation or its load, or a connect
 * that timed out from where that exit sits: another exit may well get there,
 * and Tor's connection_ap_process_end_not_open sends each of these elsewhere.
 * A refused connection (3) or a name that does not resolve (2) is the
 * destination's own answer, the same through any exit, and is passed back.
 */
/// True when an END with `reason` before CONNECTED is worth another exit.
pub fn retries(reason: u8) -> bool {
    matches!(
        reason,
        REASON_EXITPOLICY
            | REASON_TIMEOUT
            | REASON_NOROUTE
            | REASON_HIBERNATING
            | REASON_RESOURCELIMIT
    )
}

/// Whether a stream holding `held` that ended with `reason` before it was
/// connected is sent once more: for those reasons only, and only once. A
/// stream that has let go of its open, or never held one, is not.
pub fn sends_again(held: Option<&Held>, reason: u8) -> bool {
    held.is_some_and(|held| !held.again) && retries(reason)
}
