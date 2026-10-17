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
use crate::state::{FrameRepeat, Seen};

// Verbatim-repeat handling: a device generates reports at its own rate
// (~8ms) while we poll faster, so an unchanged report is processed only every
// PACE-th poll, and a run longer than MAX repeats is a stale buffer (the pad
// re-serving its last report), not input. Real finger motion changes the
// sample constantly and is untouched by either bound.
const FRAME_REPEAT_PACE: u32 = 4;
const FRAME_REPEAT_MAX: u32 = 12;

/// Stale-report suppression: polled reads return the current report whether
/// or not it is new. An unchanged sample is paced down to the device's own
/// report rate, and a long run of one is the pad re-serving a stale buffer
/// (its last motion report after finger lift, or a post-reset
/// announcement); treating those as input drifts the cursor by itself.
/// True when `seen` is to be skipped.
pub(super) fn skip_repeat(r: &mut FrameRepeat, seen: Seen) -> bool {
    if r.last == Some(seen) {
        r.frame_repeats = r.frame_repeats.saturating_add(1);
        return r.frame_repeats > FRAME_REPEAT_MAX
            || !r.frame_repeats.is_multiple_of(FRAME_REPEAT_PACE);
    }
    r.last = Some(seen);
    r.frame_repeats = 0;
    false
}
