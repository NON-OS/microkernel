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

//! The files the tests put on the volume, and the State they run in.

use crate::process::TERMINAL_PID;
use crate::term::cwd::Cwd;
use crate::term::scrollback::Scrollback;
use crate::term::state::State;

/// `n` bytes of text, a newline every 64, so line splitting has work to do.
pub fn sized(n: usize) -> Vec<u8> {
    (0..n).map(|i| if i % 64 == 63 { b'\n' } else { b'a' + (i % 26) as u8 }).collect()
}

/// A State in the home directory, its pid already resolved.
pub fn state() -> State {
    State { scrollback: Scrollback::default(), cwd: Cwd::new(), owner_pid: TERMINAL_PID }
}

/// The rows a printer that pushes each line of `data` leaves.
pub fn lines(data: &[u8]) -> Vec<(bool, Vec<u8>)> {
    data.split(|&b| b == b'\n').map(|l| (false, l.to_vec())).collect()
}

/// The one row a refusal leaves.
pub fn said(error: bool, line: &[u8]) -> Vec<(bool, Vec<u8>)> {
    vec![(error, line.to_vec())]
}
