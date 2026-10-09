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

//! Asking a proxy how far its network has got (`status`), no more than once
//! a second for each proxy and inside the tick's budget for proxy calls
//! (`pace`), so a page waiting on a cold network costs the window nothing
//! it would notice.

use alloc::vec::Vec;
use nonos_libc::mk_uptime_ms;
use spin::Mutex;

use super::call::exchange;
use super::fault::Fault;
use super::route::PACE;
use super::status::{read, Heard, STATUS_ASK};

/// How long an answer is believed before the proxy is asked again.
const FRESH_MS: i64 = 1_000;

/// The last answer from each proxy: its port, when, and what it said.
static HEARD: Mutex<Vec<(u32, i64, Heard)>> = Mutex::new(Vec::new());

/// How many exits the proxy at `port` has walked away from this session for
/// silence, asked now rather than from the last answer: a navigation reads
/// it as it starts and as it ends, and compares. `None` when the proxy does
/// not say or does not answer in the poll wait.
pub fn silent_now(port: u32) -> Option<u8> {
    let now = mk_uptime_ms();
    let got = exchange(port, &[STATUS_ASK]);
    PACE.lock().asked(port, now, mk_uptime_ms(), got.is_ok());
    let said = got.ok().and_then(|raw| read(&raw))?;
    let mut all = HEARD.lock();
    all.retain(|e| e.0 != port);
    all.push((port, now, Heard::Known(said)));
    said.silent
}

/// How far the proxy at `port` says its network has got.
pub fn heard(port: u32) -> Heard {
    let now = mk_uptime_ms();
    let last = HEARD.lock().iter().find(|e| e.0 == port).map(|e| (e.1, e.2));
    if let Some((at, said)) = last {
        if now.wrapping_sub(at) < FRESH_MS {
            return said;
        }
    }
    /* A proxy left alone after an unanswered call is busy, not silent. */
    if !PACE.lock().may_ask(port, now) {
        return last.map_or(Heard::Busy, |(_, said)| said);
    }
    let got = exchange(port, &[STATUS_ASK]);
    PACE.lock().asked(port, now, mk_uptime_ms(), got.is_ok());
    let said = match got {
        Ok(raw) => read(&raw).map_or(Heard::Unknown, Heard::Known),
        Err(Fault::Unanswered) => Heard::Busy,
        Err(Fault::Refused) => Heard::Unknown,
    };
    let mut all = HEARD.lock();
    all.retain(|e| e.0 != port);
    all.push((port, now, said));
    said
}
