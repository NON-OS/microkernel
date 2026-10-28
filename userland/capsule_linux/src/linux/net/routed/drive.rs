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

//! A socket's stream moved on. Nothing tells the family when a proxy has
//! answered, so the stream is stepped whenever the guest looks at it: its
//! connect, a read, a write, a poll, and a parked call's tick (every 10 ms,
//! family_waits_next.rs). No call out is made while the socket table is held.

use nonos_libc::mk_uptime_ms;

use super::super::sock::{self, Backend, Via};
use super::adopt::adopt;
use super::apply::{failed, opened};
use super::entry::Phase;
use super::outcome::Outcome;
use super::step::step;
use super::table;

/// Where a stream has got to, as a connect reads it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Drove {
    /// It opened in this step.
    Opened,
    /// It is still opening.
    Waiting,
    /// It did not open.
    Failed(i64),
    /// Open since an earlier step, or the SOCKS endpoint's conversation.
    Same,
}

/// The stream socket `id` names, by its index in the table.
pub fn handle(id: u32) -> Option<u32> {
    sock::with(|t| match t.get(id)?.svc.as_ref().map(Backend::via)? {
        Via::Routed(h) => Some(h),
        Via::Anon => None,
    })
}

/// Step socket `id`'s stream and apply what came of it. `asked` is set when
/// the guest's own connect asks, which then hears the outcome; any other
/// look leaves it for the next connect, or for SO_ERROR.
pub fn drive(id: u32, asked: bool) -> Drove {
    let Some(h) = handle(id) else {
        return Drove::Same;
    };
    let (now, spare) = (mk_uptime_ms(), table::free_stream());
    let Some(out) = table::with(h, |e| step(e, now, spare)) else {
        return Drove::Same;
    };
    match out {
        Outcome::Same => table::with(h, |e| match e.phase {
            Phase::Failed(errno) => Drove::Failed(errno),
            ref p if p.opening() => Drove::Waiting,
            _ => Drove::Same,
        })
        .unwrap_or(Drove::Same),
        Outcome::Opened => opened(id, h, asked),
        Outcome::Anon { port, id: sid } => adopt(id, h, port, sid, asked),
        Outcome::Failed(errno) => failed(id, h, errno, asked),
    }
}
