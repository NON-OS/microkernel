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

//! Where parked events live, and which of them may be given up.
//!
//! An interrupt-IN endpoint stays armed on one TRB until that TRB's event is
//! taken; if the event is lost the endpoint waits for good, and a keyboard
//! goes quiet. The table used to give up its oldest event when full, armed
//! or not. Now each armed transfer is named here when it is queued, its
//! event is always kept, and only events no armed transfer waits for are
//! given up, oldest first, past `PARKED` of them. Every armed transfer has at
//! most one event, so the table never needs more than `PARKED + ARMED`.

use super::issued_transfer::IssuedTransfer;
use crate::trb::Trb;

/// Events kept that no armed transfer waits for.
pub const PARKED: usize = 8;
/// Armed transfers at once: `MAX_INTERRUPT_ENDPOINTS` on eight devices.
pub const ARMED: usize = 32;
const KEPT: usize = PARKED + ARMED;

pub struct Parked {
    pub(super) armed: [Option<IssuedTransfer>; ARMED],
    pub(super) kept: [Option<(u64, Trb)>; KEPT],
    pub(super) seq: u64,
    pub(super) given_up: u64,
}

impl Parked {
    pub const fn new() -> Self {
        Self { armed: [None; ARMED], kept: [None; KEPT], seq: 0, given_up: 0 }
    }

    /// Whether `event` is the completion of a transfer named as armed.
    pub fn awaited(&self, event: &Trb) -> bool {
        self.armed.iter().flatten().any(|a| a.completed_by(event))
    }

    /// The oldest kept event no armed transfer waits for, and how many
    /// such events there are.
    pub(super) fn oldest_unawaited(&self) -> (Option<usize>, usize) {
        let mut oldest: Option<(usize, u64)> = None;
        let mut count = 0;
        for (at, entry) in self.kept.iter().enumerate() {
            let Some((seq, event)) = entry else { continue };
            if self.awaited(event) {
                continue;
            }
            count += 1;
            if oldest.is_none_or(|(_, s)| *seq < s) {
                oldest = Some((at, *seq));
            }
        }
        (oldest.map(|(at, _)| at), count)
    }
}
