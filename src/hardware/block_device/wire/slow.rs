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

//! One line for a block request that failed, or that took long enough to
//! say something about the machine: how long it waited for the wire, and
//! how long the driver took to answer once it had it. A driver that answers
//! "did not answer in time" after its own full budget reads differently from
//! one that was never run, and from one that queued behind another request.

use super::endpoint::Endpoint;
use super::slow_rule::{lost_said, said};
use crate::sys::serial::Line;

/// `[BLOCK] slow <disk>: op <op> status <s> after <a> ms, <w> ms of it for the wire`
pub(super) fn say(e: &Endpoint, op: u16, status: i32, waited_ms: u64, answered_ms: u64) {
    if !said(status, answered_ms) {
        return;
    }
    let mut line = Line::new();
    line.str(b"[BLOCK] slow ").str(e.shown.as_bytes()).str(b": op ").dec(u64::from(op));
    line.str(b" status ");
    if status < 0 {
        line.str(b"-");
    }
    line.dec(u64::from(status.unsigned_abs())).str(b" after ").dec(answered_ms);
    line.str(b" ms, ").dec(waited_ms).str(b" ms of it for the wire").end();
}

/// No answer at all: the kernel's own wait ran out, or it failed slowly.
pub(super) fn lost(e: &Endpoint, op: u16, timed_out: bool, waited_ms: u64, answered_ms: u64) {
    if !lost_said(timed_out, answered_ms) {
        return;
    }
    let mut line = Line::new();
    line.str(b"[BLOCK] slow ").str(e.shown.as_bytes()).str(b": op ").dec(u64::from(op));
    line.str(b" no answer after ").dec(answered_ms);
    line.str(b" ms, ").dec(waited_ms).str(b" ms of it for the wire").end();
}
