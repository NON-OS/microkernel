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

//! How the asking of a device moves on: to the next command at once, or to
//! the same one again after a pause (the next unit is `units`). Every wait
//! is on the clock and ends in a named failure; the server answers requests
//! in the pauses.

use super::ask::Step;
use super::asking::{Asked, Asking};
use super::report::Stage;
use crate::clock::Deadline;

/// Commands in a row the transport may fail, a REQUEST SENSE among them
/// (disk/sense.rs), on any of the device's units, before the device is
/// given up: its units share the bulk pipes, and each unit was given its own
/// tries, minutes in all on a silent card reader. Linux's SCSI scan sends
/// INQUIRY three times.
const LOST_TRIES: u8 = 3;
const PAUSE_MS: u64 = 50;

impl Asking {
    /// On to `asked` at once.
    pub(super) fn then(self, asked: Asked) -> Step {
        Step::Again(Self { asked, due: Deadline::after_ms(0), failed: 0, ..self })
    }

    /// The same command again after a pause. A transport that failed
    /// (`lost`) counts toward the tries, an answer of not yet does not; when
    /// the tries run out the device is given up with `e`.
    pub(super) fn again(mut self, e: i32, lost: bool) -> Step {
        self.failed = if lost { self.failed.saturating_add(1) } else { 0 };
        if self.failed >= LOST_TRIES {
            return Step::Failed(self.disk, Stage::NotReady, e);
        }
        self.due = Deadline::after_ms(PAUSE_MS);
        Step::Again(self)
    }
}
