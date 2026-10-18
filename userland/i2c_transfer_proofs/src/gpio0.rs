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

//! driver.gpio0 as the doorbell handler asks it: the shipping wire and its
//! Doorbell, and a look whose answer the test sets, as no gpio0 serves in
//! this crate. What the handler does with the answer is the shipping code.

use std::cell::Cell;

#[path = "../../capsule_driver_i2c_pci/src/gpio0/wire.rs"]
pub mod wire;

pub use wire::Doorbell;

thread_local! {
    static ANSWER: Cell<Option<bool>> = const { Cell::new(None) };
}

/// What every look answers from now on this thread: whether the line rang,
/// or None for a gpio0 that does not answer.
pub fn answer(rang: Option<bool>) {
    ANSWER.with(|a| a.set(rang));
}

pub fn look(_db: &Doorbell) -> Option<bool> {
    ANSWER.with(Cell::get)
}
