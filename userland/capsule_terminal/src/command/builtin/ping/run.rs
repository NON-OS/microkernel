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

//! `ping <host>`: a few echoes inline, each said, then the summary.

use super::{emit, probe, refused::refused, resolve};
use crate::command::output::Output;

// Echo requests the synchronous `ping` sends, one per sequence number, so a
// single dropped packet no longer reads as a dead host.
const PING_COUNT: u16 = 4;

pub fn run(out: &mut Output<'_>, argv: &[&[u8]]) {
    if argv.len() < 2 {
        out.writeln(b"usage: ping <host>");
        return;
    }
    if refused(out) {
        return;
    }
    let dst = match resolve::resolve(argv[1]) {
        resolve::Resolved::Ip(ip) => ip,
        resolve::Resolved::NoService => return out.writeln(b"ping: dns service unavailable"),
        resolve::Resolved::Timeout => {
            return out.writeln(b"ping: dns query timed out (no reply in 6s)")
        }
        resolve::Resolved::ServFail => return out.writeln(b"ping: dns lookup failed (servfail)"),
        resolve::Resolved::Unknown => return out.writeln(b"ping: unknown host"),
    };
    let Some(port) = probe::lookup_ip_service() else {
        out.writeln(b"ping: net unavailable");
        return;
    };
    emit::emit_target(out, argv[1], &dst);

    let mut stats = emit::Stats::new();
    for seq in 1..=PING_COUNT {
        match probe::probe(port, dst, seq) {
            probe::Probe::Reply(rtt) => {
                stats.record(rtt);
                emit::emit_reply_seq(out, &dst, seq, rtt);
            }
            probe::Probe::Timeout | probe::Probe::Unreachable => {
                stats.miss();
                emit::emit_timeout_seq(out, seq);
            }
            // A routing or configuration fault will not fix itself between
            // packets, so report it once and stop rather than repeat it four
            // times.
            fatal => {
                emit::emit_fatal(out, fatal);
                break;
            }
        }
    }
    emit::emit_summary(out, argv[1], &stats);
}
