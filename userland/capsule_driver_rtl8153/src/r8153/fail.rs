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

//! What a step of the bring-up gives back when it stops: the step's name,
//! which goes on the log line, and the errno.

/// A step that stopped, named for the owner's log, with its errno.
pub type Fail = (&'static str, i32);

/// The chip did not reach the state Linux waits for in the time Linux
/// gives it.
pub const E_TIMEDOUT: i32 = -110;
/// A device this driver does not bring up: another chip of the family, or
/// no vendor configuration.
pub const E_NODEV: i32 = -19;
/// The chip holds no station address the stack can use; Linux
/// __rtl8152_set_mac_address answers -EADDRNOTAVAIL for the same.
pub const E_ADDRNOTAVAIL: i32 = -99;

/// A register access's error, named after the step it stopped.
pub fn at<T>(what: &'static str, r: Result<T, i32>) -> Result<T, Fail> {
    r.map_err(|e| (what, e))
}
