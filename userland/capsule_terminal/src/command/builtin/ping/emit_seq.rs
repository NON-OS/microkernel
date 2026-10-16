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

//! The per-packet lines of a multi-packet ping: a reply, a drop, a fault that stops it.

use super::super::probe::Probe;
use super::fmt_ipv4;
use crate::command::output::Output;
use crate::term::util::{copy_into, format_u64};

// One reply line: "reply from 10.0.2.2: seq=1 time=3 ms".
pub fn emit_reply_seq(out: &mut Output<'_>, dst: &[u8; 4], seq: u16, rtt: u64) {
    let mut line = [0u8; 80];
    let mut k = 0;
    k += copy_into(&mut line[k..], b"reply from ");
    k += fmt_ipv4(&mut line[k..], dst);
    k += copy_into(&mut line[k..], b": seq=");
    k += format_u64(seq as u64, &mut line[k..]);
    k += copy_into(&mut line[k..], b" time=");
    k += format_u64(rtt, &mut line[k..]);
    k += copy_into(&mut line[k..], b" ms");
    out.writeln(&line[..k]);
}

// One dropped-packet line: "request timed out: seq=2".
pub fn emit_timeout_seq(out: &mut Output<'_>, seq: u16) {
    let mut line = [0u8; 48];
    let mut k = 0;
    k += copy_into(&mut line[k..], b"request timed out: seq=");
    k += format_u64(seq as u64, &mut line[k..]);
    out.writeln(&line[..k]);
}

// A fault that will not clear between packets: report it once, then stop.
pub fn emit_fatal(out: &mut Output<'_>, probe: Probe) {
    match probe {
        Probe::NoRoute => out.writeln(b"no route to host"),
        Probe::NotReady => out.writeln(b"ping: network not ready"),
        _ => out.writeln(b"ping: send failed"),
    }
}
