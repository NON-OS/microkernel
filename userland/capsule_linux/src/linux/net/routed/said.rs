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

//! Saying a stream that did not open (line.rs) where the person running
//! the program reads it: on the run's terminal, through the private write
//! that reaches only its own window and never a log. A run on no terminal
//! says it on the debug channel, which a release keeps off serial, and
//! which a terminal's run keeps to its window in every build.

use super::super::guest_route::Net;

pub fn say(net: Net, why: &str) {
    let line = super::line::line(net, why);
    if crate::linux::console::attached() {
        crate::linux::console::say(line.as_bytes());
    } else {
        crate::linux::say::say(line.as_bytes());
    }
}
