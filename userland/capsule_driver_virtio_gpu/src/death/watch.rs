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
//! The watch (G-73), run on every wake of the server loop: a dead or silent
//! owner has the screen blanked, and a blank that did not land whole is
//! tried again on the next wake. The owner's next HEALTHCHECK says LOST, so
//! one that was only slow sets its scanout again and repaints whole.

use super::blank::blank;
use crate::driver::Driver;

/// `alive`: whether the owner's process still runs. True when it blanked.
pub fn watch(d: &Driver, now: u64, alive: bool) -> bool {
    if !d.owner.due(now, alive) {
        return false;
    }
    d.owner.lost.set(true);
    let whole = blank(d);
    d.owner.quiet.set(whole);
    true
}
