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

//! What the silence means at each stage: go on, finished, or failed.

use super::{Heard, CLOSED_HANDSHAKE, QUIET_ANON, QUIET_HANDSHAKE, TOTAL_ANON, TOTAL_DIRECT};

/// Whether a stage that waits on the far end goes on, is over, or failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Settle {
    More,
    Finish,
    Fail(&'static str),
}

fn total(anonymous: bool) -> &'static str {
    if anonymous {
        TOTAL_ANON
    } else {
        TOTAL_DIRECT
    }
}

/// The response is asked to close the connection once answered. A proxy
/// says when the far end finished; a direct socket says nothing, and its
/// quiet is the end, as the blocking reader took it. Through an anonymity
/// network a stream that only went quiet stalled, and what came of it is
/// not passed off as the whole.
pub fn settle_response(heard: Heard, ended: bool, anonymous: bool) -> Settle {
    if ended {
        return Settle::Finish;
    }
    match heard {
        Heard::Waiting => Settle::More,
        Heard::Quiet if anonymous => Settle::Fail(QUIET_ANON),
        Heard::Quiet => Settle::Finish,
        Heard::Total => Settle::Fail(total(anonymous)),
    }
}

/// The handshake is over only when a whole server Finished has arrived, so
/// here neither a close nor a quiet is an ending.
pub fn settle_handshake(heard: Heard, ended: bool, anonymous: bool) -> Settle {
    if ended {
        return Settle::Fail(CLOSED_HANDSHAKE);
    }
    match heard {
        Heard::Waiting => Settle::More,
        Heard::Quiet if anonymous => Settle::Fail(QUIET_ANON),
        Heard::Quiet => Settle::Fail(QUIET_HANDSHAKE),
        Heard::Total => Settle::Fail(total(anonymous)),
    }
}
