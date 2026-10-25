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


//! What an unboxed run was refused, counted as a box counts it (tally.rs)
//! and said once when the family is done (run_line.rs). Only the serve loop
//! reaches it, as with the box itself (state.rs).

use core::cell::RefCell;

use super::kind::Kind;
use super::state::BOX;
use super::tally::Tally;

struct Run(RefCell<Option<Tally>>);

/*
 * SAFETY: the serve loop is the only thread in this capsule that reaches the
 * tally; guest threads run in their own processes and only trap into it.
 */
unsafe impl Sync for Run {}

static RUN: Run = Run(RefCell::new(None));

pub(super) fn call_begins() {
    RUN.0.borrow_mut().get_or_insert_with(Tally::default).call_begins();
}

pub(super) fn note(kind: Kind, example: &[u8]) {
    RUN.0.borrow_mut().get_or_insert_with(Tally::default).note(kind, example);
}

/// Said once, when the family is done; a boxed run's report says it instead.
pub fn report() {
    let Some(tally) = RUN.0.borrow_mut().take() else { return };
    if BOX.0.borrow().is_some() {
        return;
    }
    if let Some(line) = super::run_line::run_line(&tally) {
        crate::linux::say::say(line.as_bytes());
    }
}
