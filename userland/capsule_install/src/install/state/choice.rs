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

//! The confirm screen's two rows, on a disk an update could keep: update
//! NØNOS and keep the data, or erase the disk and install, as on any other
//! disk. A disk whose data is not ready shows the update row and lets it be
//! looked at, for how to get it ready, but plans nothing from it.

use nonos_disk::Keep;

use super::machine::State;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Choice {
    Update,
    Erase,
}

impl State {
    /// The chosen disk is one an update could keep: the two rows show.
    pub fn offers_update(&self) -> bool {
        matches!(self.keep, Some(Keep::Ready(_) | Keep::NeedsPrepare(_)))
    }

    /// The update row is the one lit, ready or not.
    pub fn on_update_row(&self) -> bool {
        self.offers_update() && self.choice == Choice::Update
    }

    /// The chosen disk's data is one a new release opens, so it can be kept.
    pub fn keep_ready(&self) -> bool {
        matches!(self.keep, Some(Keep::Ready(_)))
    }

    /// An update is what is planned, written or done: the update row on a
    /// disk whose data is ready to keep.
    pub fn updating(&self) -> bool {
        self.choice == Choice::Update && self.keep_ready()
    }

    /// The screen's title, which says update rather than erase on the
    /// update row.
    pub fn title(&self) -> &'static str {
        if self.on_update_row() {
            self.screen.kept_title(self.updating())
        } else {
            self.screen.title()
        }
    }
}
