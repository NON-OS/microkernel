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

//! How the session reaches the PPM: the caller maps the mailbox and runs
//! the device's _DSM, so this crate does no I/O of its own.

pub trait Ppm {
    fn write_control(&mut self, control: u64) -> bool;
    fn read_cci(&mut self) -> Option<u32>;
    fn read_message(&mut self, out: &mut [u8; 16]) -> bool;
    /// Tell the PPM the OPM wrote CONTROL (the _DSM's write function).
    fn signal_write(&mut self) -> bool;
    /// Ask firmware to bring the mailbox up to date (the _DSM's read).
    fn signal_read(&mut self) -> bool;
    /// Let one poll interval pass.
    fn pause(&mut self);
}
