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

//! The line one finished probe says, and the exit status it leaves.

use super::super::probe::Probe;
use super::emit_reply;
use crate::command::output::Output;

// Shared terminal outcome for a finished probe: writes the same line the
// synchronous `ping` builtin always has, and returns an exit status.
pub fn emit_probe(out: &mut Output<'_>, dst: [u8; 4], probe: Probe) -> i32 {
    match probe {
        Probe::Reply(rtt) => {
            emit_reply(out, &dst, rtt);
            0
        }
        Probe::NoRoute => {
            out.writeln(b"no route to host");
            1
        }
        Probe::NotReady => {
            out.writeln(b"ping: network not ready");
            1
        }
        Probe::Unreachable => {
            out.writeln(b"ping: destination unreachable (no ARP reply)");
            1
        }
        Probe::Timeout => {
            out.writeln(b"request timed out");
            1
        }
        Probe::SendFailed => {
            out.writeln(b"ping: send failed");
            1
        }
    }
}
