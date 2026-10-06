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
 * When a download whose connection failed or dropped is tried again, and
 * when it is given up, and as what. Every try is a new connection on a new
 * conversation through the route (a fresh SOCKS handshake for Nym), asking
 * the mirror for the bytes not yet on the volume only.
 *
 * Through Nym or Anyone a failure is most often an exit that went silent:
 * net.socks5 rotates to the next exit and ends every stream it carried, and
 * a request right after that is refused until a new connection opens a
 * session on the next exit. So an anonymity network gets up to `ROTATIONS`
 * tries in a row with no byte between them, backing off from 5 s to 30 s,
 * and no more than `PATIENCE_MS` of them; any byte that comes starts the
 * count again. A direct connection keeps the rounds over every mirror it
 * had, a second more each time.
 *
 * Given up, the end says why, each its own sentence and exit status: the
 * network had no session at all and no connection opened (`Unreachable`);
 * through an anonymity network no exit answered (`NoExit`), which offers a
 * direct download; directly, no mirror served the file (`NoMirror`), or
 * none was reached at all (`Unreachable`). Pure, so model_fetch_proofs
 * holds the counts, the waits and the ends.
 */

/* What the last failure was, as the route said it. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /* The anonymity network has no session to open a stream in. */
    NoSession,
    /* A connection or stream that failed or dropped otherwise. */
    Other,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum End {
    Unreachable,
    NoExit,
    NoMirror,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Next {
    /* Try again after `ms`; this is try `n` of at most `of` in a row. */
    Wait { ms: i64, n: u32, of: u32 },
    GiveUp(End),
}

/* Tries in a row through an anonymity network, and how long they may take in all. */
pub const ROTATIONS: u32 = 6;
pub const PATIENCE_MS: i64 = 180_000;
/* Rounds over every mirror for a direct connection. */
pub const ROUNDS: u32 = 3;

pub struct Retry {
    tries: u32,
    since_ms: i64,
    /* A connection to a mirror opened at some point this run. */
    pub opened: bool,
    /* Every failure so far was the network having no session. */
    only_no_session: bool,
}

impl Retry {
    pub fn new(now_ms: i64) -> Retry {
        Retry { tries: 0, since_ms: now_ms, opened: false, only_no_session: true }
    }

    /*
     * After a failure of `kind`: whether bytes came since the last one
     * (`progressed`), whether any came this run (`got_any`), through an
     * anonymity network or not, over `mirrors` mirrors.
     */
    pub fn after(
        &mut self,
        anonymous: bool,
        mirrors: usize,
        progressed: bool,
        got_any: bool,
        kind: Kind,
        now_ms: i64,
    ) -> Next {
        self.only_no_session &= kind == Kind::NoSession;
        if progressed {
            (self.tries, self.since_ms) = (0, now_ms);
        }
        self.tries += 1;
        let (of, out_of_time) = match anonymous {
            true => (ROTATIONS, now_ms - self.since_ms >= PATIENCE_MS),
            false => (ROUNDS * mirrors.max(1) as u32, false),
        };
        if self.tries < of && !out_of_time {
            let ms = match anonymous {
                true => (5_000i64 << (self.tries - 1).min(3)).min(30_000),
                false => 1_000 * i64::from(self.tries.min(10)),
            };
            return Next::Wait { ms, n: self.tries + 1, of };
        }
        let nothing = !self.opened && !got_any;
        Next::GiveUp(match (anonymous, nothing) {
            (true, true) if self.only_no_session => End::Unreachable,
            (true, _) => End::NoExit,
            (false, true) => End::Unreachable,
            (false, false) => End::NoMirror,
        })
    }
}
