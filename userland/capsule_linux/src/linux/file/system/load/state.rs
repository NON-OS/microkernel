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

/* The load's state: when it was last taken, and what had run by then. */

use core::cell::RefCell;

pub(super) struct State {
    pub(super) at_ms: u64,
    pub(super) ran: u64,
    pub(super) gone: u64,
    pub(super) avg: [u64; 3],
}

pub(super) struct Load(pub(super) RefCell<State>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Load {}

pub(super) static LOAD: Load = Load(RefCell::new(State { at_ms: 0, ran: 0, gone: 0, avg: [0; 3] }));

/* A process of the family ran `ticks` in all before it exited. */
pub fn exited(ticks: u64) {
    LOAD.0.borrow_mut().gone += ticks;
}
