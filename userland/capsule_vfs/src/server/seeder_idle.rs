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

//! Advancing the staging load from the receive loop's idle slot.

use crate::store::Store;

use super::seeder::{PackageSeeder, QUIET_POLLS};

impl PackageSeeder {
    pub fn on_idle(&mut self, store: &mut Store) {
        if self.done {
            return;
        }
        /*
         * The quiet gate is for starting a load. One in progress takes every
         * idle slot, and poll_ms makes those a millisecond apart: gated, a
         * slice ran every 750 ms and staging took ten minutes under TCG.
         */
        if self.load.is_none() {
            self.quiet += 1;
            if self.quiet < QUIET_POLLS + self.attempts {
                return;
            }
            self.quiet = 0;
        }
        self.advance(store);
    }
}
