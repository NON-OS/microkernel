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

//! How long an exchange waits on the far end, judged once a tick instead of
//! inside a read loop. Two bounds, as the blocking readers had them: quiet,
//! which every byte that arrives restarts, and total, which nothing restarts,
//! so a drip of one byte every few seconds cannot hold a job open forever.

/// A direct socket's quiet: a server silent this long has finished, or gone.
pub const DIRECT_QUIET_MS: i64 = 4_000;
/// A whole exchange on a direct socket.
pub const DIRECT_TOTAL_MS: i64 = 120_000;
/// Through an anonymity network seconds of quiet are normal: a reply crosses
/// several relays each way, the mixnet delaying each packet on purpose.
pub const ANON_QUIET_MS: i64 = 60_000;
/// A whole exchange through an anonymity network.
pub const ANON_TOTAL_MS: i64 = 600_000;

/// What the clock says about the far end this tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Heard {
    /// Still within both bounds.
    Waiting,
    /// Nothing arrived for the quiet bound.
    Quiet,
    /// The whole exchange ran past the total bound.
    Total,
}

#[derive(Clone, Copy, Debug)]
pub struct Wait {
    quiet_ms: i64,
    quiet_until: i64,
    total_until: i64,
}

impl Wait {
    pub fn new(now_ms: i64, anonymous: bool) -> Self {
        let (quiet_ms, total_ms) = if anonymous {
            (ANON_QUIET_MS, ANON_TOTAL_MS)
        } else {
            (DIRECT_QUIET_MS, DIRECT_TOTAL_MS)
        };
        Wait {
            quiet_ms,
            quiet_until: now_ms.saturating_add(quiet_ms),
            total_until: now_ms.saturating_add(total_ms),
        }
    }

    /// Bytes arrived: the quiet bound starts again from now.
    pub fn heard(&mut self, now_ms: i64) {
        self.quiet_until = now_ms.saturating_add(self.quiet_ms);
    }

    pub fn check(&self, now_ms: i64) -> Heard {
        if now_ms >= self.total_until {
            return Heard::Total;
        }
        if now_ms >= self.quiet_until {
            return Heard::Quiet;
        }
        Heard::Waiting
    }
}

/// Whether a stage that waits on the far end goes on, is over, or failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Settle {
    More,
    Finish,
    Fail(&'static str),
}

pub const QUIET_ANON: &str = "nothing came back for 60 seconds";
pub const QUIET_HANDSHAKE: &str = "the server went quiet during the handshake";
pub const CLOSED_HANDSHAKE: &str = "the connection closed during the handshake";
pub const TOTAL_DIRECT: &str = "gave up after 2 minutes";
pub const TOTAL_ANON: &str = "gave up after 10 minutes";

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
