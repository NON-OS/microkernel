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

//! The report a boxed run ends with.

use super::state::BOX;

/// The run ended with `exit`: its report where the person reads it, and its
/// line in the serial log for automation. Nothing for an unboxed run.
pub fn finish(exit: i32) {
    let Some(a) = BOX.0.borrow_mut().take() else { return };
    let text = super::report::report(&a.policy, &a.hash, &a.tally, exit);
    match crate::linux::console::attached() {
        true => crate::linux::console::say(text.as_bytes()),
        false => crate::linux::say::say(text.as_bytes()),
    }
    crate::linux::say::say(super::line::box_line(&a.policy, &a.hash, &a.tally, exit).as_bytes());
}
