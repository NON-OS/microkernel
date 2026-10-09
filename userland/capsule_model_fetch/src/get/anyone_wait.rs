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
 * Waiting for the Anyone network before a download, rather than failing.
 * Installs download over Anyone (nonos_route_link `install_route`), and on
 * a cold boot net.anon takes minutes to build its first circuit: until then
 * it refuses a stream as not connected yet. So the fetcher asks net.anon how
 * far it has got (the status ask, one byte 6, answered [3, 1, ready, step,
 * steps]; capsule_net_anon server/socks/progress.rs), says it ("Anyone is
 * building its circuit, step 3 of 5"), and goes on once it is ready, or
 * gives up after `HOLD_MS` with Retry and a direct download offered. A
 * net.anon too old to answer the ask is taken as ready, and the stream's
 * own refusal says the rest. Pure, so model_fetch_proofs holds it.
 */

use alloc::format;
use alloc::string::String;

/* The status ask, and the longest a download waits for Anyone. */
pub const STATUS_ASK: u8 = 6;
pub const HOLD_MS: i64 = 180_000;
/* Between two asks. */
pub const GAP_MS: i64 = 1_000;

/* How far the Anyone network has got, as net.anon says it. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Heard {
    pub ready: bool,
    pub step: u8,
    pub steps: u8,
}

/* The answer in `raw`, or None for bytes that are not one. */
pub fn read(raw: &[u8]) -> Option<Heard> {
    let &[3, 1, ready, step, steps] = raw else { return None };
    let sane = ready <= 1 && (1..=8).contains(&steps) && (1..=steps).contains(&step);
    sane.then_some(Heard { ready: ready == 1, step, steps })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Wait {
    /* Download now. */
    Go,
    /* Ask again after GAP_MS; how far it has got, if it said. */
    Again(Option<Heard>),
    /* HOLD_MS passed with Anyone not up. */
    GiveUp,
}

/*
 * What to do, with net.anon registered or not, what it answered the ask
 * (None for no answer, or one that is not a status), and how long since
 * the wait began.
 */
pub fn decide(registered: bool, heard: Option<Heard>, answered: bool, waited_ms: i64) -> Wait {
    match heard {
        Some(h) if registered && h.ready => return Wait::Go,
        /* An answer that is not a status: a net.anon older than the ask. */
        None if registered && answered => return Wait::Go,
        _ => {}
    }
    if waited_ms >= HOLD_MS {
        return Wait::GiveUp;
    }
    Wait::Again(heard.filter(|_| registered))
}

/* "Anyone is building its circuit, step 3 of 5", or that it is starting. */
pub fn line(heard: Option<Heard>) -> String {
    match heard {
        Some(h) => format!("Anyone is building its circuit, step {} of {}", h.step, h.steps),
        None => String::from("Anyone is starting"),
    }
}
