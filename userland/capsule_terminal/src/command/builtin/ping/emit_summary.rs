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

//! The closing statistics of a multi-packet ping.

use super::Stats;
use crate::command::output::Output;
use crate::term::util::{copy_into, format_u64};

// The closing summary: transmitted/received/loss, and the round-trip spread when
// at least one reply came back.
pub fn emit_summary(out: &mut Output<'_>, host: &[u8], stats: &Stats) {
    let mut head = [0u8; 96];
    let mut k = 0;
    k += copy_into(&mut head[k..], b"--- ");
    k += copy_into(&mut head[k..], host);
    k += copy_into(&mut head[k..], b" ping statistics ---");
    out.writeln(&head[..k]);

    let mut counts = [0u8; 80];
    let mut k = 0;
    k += format_u64(stats.sent as u64, &mut counts[k..]);
    k += copy_into(&mut counts[k..], b" transmitted, ");
    k += format_u64(stats.recv as u64, &mut counts[k..]);
    k += copy_into(&mut counts[k..], b" received, ");
    let loss =
        if stats.sent > 0 { (stats.sent - stats.recv) as u64 * 100 / stats.sent as u64 } else { 0 };
    k += format_u64(loss, &mut counts[k..]);
    k += copy_into(&mut counts[k..], b"% loss");
    out.writeln(&counts[..k]);

    if stats.recv > 0 {
        let avg = stats.sum / stats.recv as u64;
        let mut rtt = [0u8; 80];
        let mut k = 0;
        k += copy_into(&mut rtt[k..], b"rtt min/avg/max = ");
        k += format_u64(stats.min, &mut rtt[k..]);
        rtt[k] = b'/';
        k += 1;
        k += format_u64(avg, &mut rtt[k..]);
        rtt[k] = b'/';
        k += 1;
        k += format_u64(stats.max, &mut rtt[k..]);
        k += copy_into(&mut rtt[k..], b" ms");
        out.writeln(&rtt[..k]);
    }
}
