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

//! A CPU handing its deadline to the frame before its own timer stops, and
//! taking it back after.

use super::state::{BOOK, FRAME, OWNER};
use crate::arch::aarch64::timer::PhysicalTimer;

/// Whether a CPU may enter a state that stops its timer: the frame is up
/// and has an owner (the owner itself may not; `owner`).
pub fn ready() -> bool {
    FRAME.get().is_some() && OWNER.get().is_some()
}

/// The roster CPU the frame's interrupt is aimed at.
pub fn owner() -> Option<usize> {
    OWNER.get().copied()
}

/// Before roster CPU `cpu` enters a state that stops its timer: its
/// deadline, `lead` ticks early so it is up by then, goes to the frame.
/// False when it cannot be kept, and then that state must not be entered.
/// Called with IRQs masked.
pub fn hand_off(cpu: usize, lead: u64) -> bool {
    let Some(frame) = FRAME.get() else {
        return false;
    };
    let armed = PhysicalTimer::is_armed();
    let due = armed.then(|| PhysicalTimer::get_compare_value().saturating_sub(lead));
    let mut book = BOOK.lock();
    match book.sleep(cpu, due) {
        Ok(next) => {
            frame.hold(next);
            true
        }
        Err(_) => false,
    }
}

/// After the state, however it ended: this CPU's own timer runs again.
pub fn take_back(cpu: usize) {
    if let Some(frame) = FRAME.get() {
        let mut book = BOOK.lock();
        frame.hold(book.wake(cpu));
    }
}
