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
 * The Anyone network made ready for a download, or the refusal when it does
 * not come up in time (`anyone_wait` decides). While it waits, the line at
 * the Terminal and the store's card say how far it has got.
 */

use nonos_libc::{mk_idle_ms, mk_ipc_call_timeout, mk_uptime_ms};
use nonos_socket::lookup;

use super::anyone_wait::{decide, line, read, Heard, Wait, GAP_MS, STATUS_ASK};
use super::refusal::Refusal;
use crate::net::Route;
use crate::out::{over, say};
use crate::serve;
use crate::status_wire::{Status, ANYONE, ANYONE_WAIT};

/* A status answer is five bytes; a net.anon busy in a call of its own is given this long. */
const ASK_MS: u64 = 2_000;

/*
 * Anyone's route once net.anon has a circuit, for a tier of `total` bytes
 * of which `done` are on the volume.
 */
pub fn ready(total: u64, done: u64) -> Result<Route, Refusal> {
    let since = mk_uptime_ms();
    let mut shown = false;
    loop {
        let port = lookup(b"net.anon");
        let (heard, answered) = if port == 0 { (None, false) } else { ask(port) };
        match decide(port != 0, heard, answered, mk_uptime_ms() - since) {
            Wait::Go => {
                if shown {
                    say("");
                }
                return Ok(Route::Anon(port));
            }
            Wait::GiveUp => {
                if shown {
                    say("");
                }
                return Err(Refusal::anyone_not_up());
            }
            Wait::Again(h) => {
                over(&alloc::format!("  {}", line(h)));
                shown = true;
                publish(h, total, done);
                pause();
            }
        }
    }
}

/* Ask net.anon how far it has got: what it said, and whether it answered at all. */
fn ask(port: u32) -> (Option<Heard>, bool) {
    let mut rx = [0u8; 16];
    let n = mk_ipc_call_timeout(port as u64, [STATUS_ASK].as_ptr(), 1, rx.as_mut_ptr(), rx.len(), ASK_MS);
    match usize::try_from(n) {
        Ok(n) if n > 0 => (read(&rx[..n.min(rx.len())]), true),
        _ => (None, false),
    }
}

/* What the store's card is told while it waits. */
fn publish(h: Option<Heard>, total: u64, done: u64) {
    let (try_n, tries) = h.map_or((0, 5), |h| (h.step, h.steps));
    serve::set(Status { stage: ANYONE_WAIT, route: ANYONE, total, done, rate: 0, try_n, tries });
}

fn pause() {
    let until = mk_uptime_ms().saturating_add(GAP_MS);
    while mk_uptime_ms() < until {
        serve::answer();
        mk_idle_ms(50);
    }
}
