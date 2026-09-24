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

//! How a closed stream is reported.

use crate::protocol::{E_STREAM_CLOSED, HDR_LEN};
use crate::stream::{is_clean, needs_another_exit};

/*
 * The close carries the reason and two flags: whether trying the same exit again
 * is pointless, and whether the far end finished cleanly. A front end that cannot
 * tell a policy refusal from a timeout either rotates away from a working exit or
 * hammers one that will never serve this destination, and one that cannot see a
 * clean finish reports a completed page as a broken one.
 */
pub(super) fn closed(reason: u8, tx: &mut [u8]) -> (u16, u32) {
    tx[HDR_LEN] = reason;
    tx[HDR_LEN + 1] = u8::from(needs_another_exit(reason));
    tx[HDR_LEN + 2] = u8::from(is_clean(reason));
    (E_STREAM_CLOSED, 3)
}
