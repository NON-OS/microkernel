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

//! `ping <host>` as a job: the host resolved and announced here, the echo handed to a `PingJob`.

use super::{emit, probe, refused::refused, resolve, PingJob};
use crate::command::output::Output;

// Job-submission variant of `run`: same host resolution and target
// announcement, but hands the echo-request/echo-reply poll off as a
// `PingJob` instead of looping it to completion inline.
pub fn prepare(out: &mut Output<'_>, argv: &[&[u8]]) -> Option<PingJob> {
    if argv.len() < 2 {
        out.writeln(b"usage: ping <host>");
        return None;
    }
    if refused(out) {
        return None;
    }
    let dst = match resolve::resolve(argv[1]) {
        resolve::Resolved::Ip(ip) => ip,
        resolve::Resolved::NoService => {
            out.writeln(b"ping: dns service unavailable");
            return None;
        }
        resolve::Resolved::Timeout => {
            out.writeln(b"ping: dns query timed out (no reply in 6s)");
            return None;
        }
        resolve::Resolved::ServFail => {
            out.writeln(b"ping: dns lookup failed (servfail)");
            return None;
        }
        resolve::Resolved::Unknown => {
            out.writeln(b"ping: unknown host");
            return None;
        }
    };
    let Some(port) = probe::lookup_ip_service() else {
        out.writeln(b"ping: net unavailable");
        return None;
    };
    emit::emit_target(out, argv[1], &dst);
    Some(PingJob::new(port, dst, 1))
}
