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

//! Telling whether a GET_ENCAPSULATED_RESPONSE answer is the completion of
//! the request just sent, as Linux rndis_command matches it: the type with
//! the completion bit, then the RequestID. Anything else (an indication,
//! a stale completion, QEMU's one zero byte while nothing is queued) is
//! passed over and the channel is polled again.

use super::message::{le32, MSG_INDICATE};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Answer {
    /// The completion: its status word and its MessageLength, which is
    /// at least 16 and no more than the bytes that came.
    Done { status: u32, len: usize },
    /// An INDICATE_STATUS message, skipped as Linux logs and skips it.
    Indication,
    /// Not ours: too short, another request's, or of another type.
    Other,
}

pub fn answer(r: &[u8], kind: u32, id: u32) -> Answer {
    let (Some(t), Some(len)) = (le32(r, 0), le32(r, 4)) else { return Answer::Other };
    if t == MSG_INDICATE {
        return Answer::Indication;
    }
    let len = len as usize;
    if t != kind || len < 16 || len > r.len() || le32(r, 8) != Some(id) {
        return Answer::Other;
    }
    match le32(r, 12) {
        Some(status) => Answer::Done { status, len },
        None => Answer::Other,
    }
}
