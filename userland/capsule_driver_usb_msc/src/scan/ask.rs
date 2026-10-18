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

//! One step of asking a configured device's logical units: the command its
//! asking is at (`command`), and what a medium change said under it means.

use super::asking::Asking;
use super::identity::Identity;
use super::report::Stage;
use crate::disk::Disk;
use crate::protocol::E_NODEV;
use crate::state::{Said, State};

/// What a step of the asking came to.
pub enum Step {
    Again(Asking),
    Ours(Disk, Identity),
    /// Not taken, with the errno its last unit gave up with: `E_NOMEDIUM`
    /// for an empty one. Its slot is still held. At `Stage::Changed` the
    /// unit served was found changed or without its card, and serving ends
    /// (`asked`).
    Failed(Disk, Stage, i32),
}

impl Asking {
    /// One step, `served` naming the device served this boot, if any. A
    /// medium change said by the unit served, asked again after a let-go,
    /// ends serving: its card may be another of the same size, and nothing
    /// tells them apart. It is judged at once, so an asking that fails after
    /// it does not leave a later one, which hears no change, to serve the
    /// card. One said by another unit or device is that one's and goes, as
    /// does one said before any device was served: a card just put in. Any
    /// other attention goes too: the unit served, served again, raises the
    /// generation the kernel reads (`asked`).
    pub fn step(self, state: &mut State, served: Option<Identity>) -> Step {
        let (disk, serial) = (self.disk, self.serial);
        let step = self.command(state, served);
        let changed = state.take_said() == Said::Changed;
        match served {
            Some(s) if changed && s.is_unit(serial, disk.lun) => {
                Step::Failed(disk, Stage::Changed, E_NODEV)
            }
            _ => step,
        }
    }
}
