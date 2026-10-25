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

/*
 * The wallet's one ASYNC request to settle (plan/28, v1.4.53): BROADCAST,
 * READ or INTENT, sent without waiting. Settle answers into the wallet's
 * inbox; app_skeleton hands every frame not from the router to peer_frame,
 * and only the answer to the request outstanding is believed: from
 * settle's registry pid, on its wire, with that request's op and id. A
 * frame that is none of those is dropped unread. No answer in STEP_MS
 * ends the request ETIME, and a late answer is then dropped too.
 */

use alloc::vec::Vec;

use nonos_settle_proto::{async_answer, parse_header, ReplyError};

/// As long as the meter waits on one settle step (v1.4.15).
pub const STEP_MS: u64 = 150_000;

/// The request outstanding: at most one per wallet.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Asked {
    pub op: u16,
    pub id: u32,
    pub settle_pid: u32,
    pub at_ms: u64,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Heard {
    /// Not settle's answer to the request outstanding: dropped unread.
    Dropped,
    /// The answer: its body, or settle's status.
    Answer(Result<Vec<u8>, i32>),
}

/// What a frame from `sender` is to the request outstanding.
pub fn heard(asked: Option<&Asked>, sender: u32, frame: &[u8]) -> Heard {
    let Some(a) = asked else { return Heard::Dropped };
    if parse_header(frame).is_none() {
        return Heard::Dropped;
    }
    match async_answer(frame, sender, a.settle_pid, a.op, a.id) {
        Ok(body) => Heard::Answer(Ok(body.to_vec())),
        Err(ReplyError::Status(s)) => Heard::Answer(Err(s)),
        Err(ReplyError::Foreign | ReplyError::Stale) => Heard::Dropped,
    }
}

/// Whether the request outstanding has waited STEP_MS.
pub fn expired(a: &Asked, now_ms: u64) -> bool {
    now_ms.saturating_sub(a.at_ms) >= STEP_MS
}
