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

//! Why a stream ended, from END_STREAM_REASON_* in or.h.

pub const REASON_MISC: u8 = 1;
pub const REASON_EXITPOLICY: u8 = 4;
pub const REASON_DONE: u8 = 6;
pub const REASON_NOROUTE: u8 = 8;
pub const REASON_HIBERNATING: u8 = 9;

/// True when the far end finished cleanly, which is not a failure at all.
///
pub fn is_clean(reason: u8) -> bool {
    reason == REASON_DONE
}

/// The reason byte an END body carries, or `MISC` when it carries none.
///
pub fn reason(body: &[u8]) -> u8 {
    body.first().copied().unwrap_or(REASON_MISC)
}

/*
 * The distinction that matters to a caller: whether to try the same exit again
 * or pick a different one. A policy refusal or an unreachable network will not
 * improve on retry through the same exit, while a timeout or a resource limit
 * might.
 */
/// True when the exit will not serve this destination however often it is asked.
pub fn needs_another_exit(reason: u8) -> bool {
    matches!(reason, REASON_EXITPOLICY | REASON_NOROUTE | REASON_HIBERNATING)
}
