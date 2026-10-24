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

//! What a test reads back of the registry, and the calls it cues to refuse.

use std::cell::Cell;

use crate::mem::{MAPS, REFUSE, WRONG};
use crate::registry::{Entry, DIRTY, LIVE, NO_REGISTER, NO_SHARE};
use crate::service::set_compositor;

/// Every surface still registered.
pub fn live() -> Vec<Entry> {
    LIVE.with(|l| l.borrow().clone())
}

pub fn refuse_registers(on: bool) {
    NO_REGISTER.with(|c| c.set(on));
}

pub fn refuse_shares(on: bool) {
    NO_SHARE.with(|c| c.set(on));
}

/// A fresh kernel for a test: nothing mapped or registered, nothing cued,
/// and the compositor registered.
pub fn reset() {
    MAPS.with(|m| m.borrow_mut().clear());
    LIVE.with(|l| l.borrow_mut().clear());
    [&REFUSE, &NO_REGISTER, &NO_SHARE].iter().for_each(|c| c.with(|c| c.set(false)));
    [&WRONG, &DIRTY].iter().for_each(|c| c.with(|c| c.set(0)));
    set_compositor(Some(0x27));
}

/// Surfaces registered, since the reset, with a byte other than zero in
/// them: each would have been shared holding what its pages held.
pub fn registered_dirty() -> u32 {
    DIRTY.with(Cell::get)
}
