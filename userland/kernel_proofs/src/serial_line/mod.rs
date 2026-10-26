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

//! A kernel line reaches the serial port whole: built in a Line and handed
//! over in one write under the port's lock, so no other CPU's line lands
//! inside it. The kernel's own line.rs, over a port that records each write.

#[cfg(test)]
mod print {
    use std::sync::Mutex;

    /// Every write the port took, in order, each under the one lock.
    pub static WRITES: Mutex<Vec<Vec<u8>>> = Mutex::new(Vec::new());

    pub fn line(s: &[u8]) {
        WRITES.lock().unwrap().push(s.to_vec());
    }
}

/// The fatal writer's port side, which line.rs's end_fatal hands to; its
/// rule is fatal.rs's, proved in fatal_tests.rs.
#[cfg(test)]
mod core {
    pub fn write_fatal_line(s: &[u8]) {
        super::print::line(s);
    }
}

#[cfg(test)]
#[allow(dead_code)]
#[path = "../../../../src/sys/serial/line.rs"]
mod line;

#[cfg(test)]
#[path = "../../../../src/sys/serial/column.rs"]
mod column;

#[cfg(test)]
#[path = "../../../../src/sys/serial/fatal.rs"]
mod fatal;

#[cfg(test)]
mod column_tests;
#[cfg(test)]
mod fatal_tests;
#[cfg(test)]
mod fault_order_tests;
#[cfg(test)]
mod lost_tests;
#[cfg(test)]
mod tests;
