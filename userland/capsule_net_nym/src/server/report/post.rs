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

//! Posting it, at a steady pace and on every change of stage.

use core::ptr;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup, mk_time_millis};
use nonos_route_proof::{post_frame, reply_body, ATTEST_SERVICE, OP_ROUTE_REPORT, POST_LEN};

use super::build::build;

/// A report at least this often, so the board never shows one past its
/// freshness window while the transport is alive.
const EVERY_MS: i64 = 5_000;
/// How long a post may hold the serve loop. The board answers as soon as it
/// is scheduled, which under a loaded or emulated machine can take past
/// 20 ms: one post in twelve went unanswered at 20 ms on a TCG boot. At most
/// this once every `EVERY_MS`; a board slower still is skipped until the
/// next turn rather than waited on.
const REPLY_MS: u64 = 100;

#[derive(Default)]
pub struct Reporter {
    port: u32,
    last_ms: i64,
    last_stage: Option<u8>,
    request_id: u32,
}

/// Post a report when one is due: every `EVERY_MS`, and at once when the
/// stage changes. Never blocks the loop longer than `REPLY_MS`, and a board
/// that is missing or refuses changes nothing about the transport.
pub fn report_due(r: &mut Reporter) {
    let now_ms = mk_time_millis();
    if now_ms < 0 {
        return;
    }
    let report = build(now_ms as u64);
    let stage = report.stage as u8;
    if r.last_stage == Some(stage) && now_ms.wrapping_sub(r.last_ms) < EVERY_MS {
        return;
    }
    r.last_ms = now_ms;
    r.last_stage = Some(stage);
    if r.port == 0 && !lookup(&mut r.port) {
        return;
    }
    r.request_id = r.request_id.wrapping_add(1);
    let frame = post_frame(&report, r.request_id);
    let mut reply = [0u8; POST_LEN];
    let n = mk_ipc_call_timeout(
        r.port as u64,
        frame.as_ptr(),
        frame.len(),
        reply.as_mut_ptr(),
        reply.len(),
        REPLY_MS,
    );
    let answered = n > 0
        && reply_body(&reply[..(n as usize).min(reply.len())], OP_ROUTE_REPORT, r.request_id)
            .is_some();
    if !answered {
        // Looked up again next time: the service may have restarted elsewhere.
        r.port = 0;
    }
}

fn lookup(port: &mut u32) -> bool {
    let rc = mk_service_lookup(
        ATTEST_SERVICE.as_ptr(),
        ATTEST_SERVICE.len(),
        port as *mut u32,
        ptr::null_mut(),
    );
    rc == 0 && *port != 0
}
