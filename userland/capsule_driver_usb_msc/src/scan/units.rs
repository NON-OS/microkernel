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

//! Passing from one logical unit of a device to the next: past one with no
//! medium, once the device has had its second to start a card, and past one
//! that is not the unit served (`sized`). After the last the device is not
//! taken.

use super::ask::Step;
use super::asking::{Asked, Asking};
use super::identity::Identity;
use super::report::Stage;
use crate::clock::Deadline;
use crate::disk::Disk;
use crate::protocol::E_NOMEDIUM;

impl Asking {
    /// A unit with no medium: asked again once the device has had its second
    /// to start a card, then passed. The unit served, asked again after a
    /// let-go with no medium in it, ends serving instead: its card was taken
    /// out, and the one put back may be another of the same size (`release`).
    pub(super) fn no_medium(mut self, served: Option<Identity>) -> Step {
        if !self.settle.expired() {
            (self.due, self.failed) = (self.settle, 0);
            return Step::Again(self);
        }
        if served.is_some_and(|s| s.is_unit(self.serial, self.disk.lun)) {
            return Step::Failed(self.disk, Stage::Changed, E_NOMEDIUM);
        }
        self.next_unit(Stage::NoMedium, E_NOMEDIUM)
    }

    /// On to the next unit, from its INQUIRY; after the last, the device is
    /// given up at `stage` with `e`.
    pub(super) fn next_unit(self, stage: Stage, e: i32) -> Step {
        match self.disk.lun.checked_add(1).filter(|&lun| lun <= self.last_lun) {
            Some(lun) => Step::Again(Self {
                disk: Disk { lun, ..self.disk },
                asked: Asked::Inquiry,
                named: None,
                due: Deadline::after_ms(0),
                failed: 0,
                ..self
            }),
            None => Step::Failed(self.disk, stage, e),
        }
    }
}
