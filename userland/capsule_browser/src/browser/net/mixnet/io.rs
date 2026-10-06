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

//! Writing to and reading from a proxy as the socket calls expect, with no
//! call waiting longer than the poll wait.

use nonos_libc::mk_uptime_ms;

use super::call::exchange;
use super::conv::Broke;
use super::fault::Fault;
use super::frames::reset;
use super::route::{pending_reset, reset_done, with, PACE};
use crate::browser::net::Recv;

/// A tick of the fetch machine begins: its calls to proxies get their whole
/// budget (`pace`), and one stream let go is told to its proxy.
pub fn new_tick() {
    PACE.lock().new_tick(mk_uptime_ms());
    tell_reset();
}

/// Ask a proxy to end a stream the browser let go, so its exit stops
/// sending for a fetch that is gone. The close did not wait for this: a
/// page left with many fetches in flight would have held the window for a
/// poll wait each. A reset left unanswered is asked again on a later tick;
/// a stream taken again before then is reset by its own opening.
fn tell_reset() {
    let Some((port, stream)) = pending_reset() else { return };
    let from = mk_uptime_ms();
    if !PACE.lock().may_ask(port, from) {
        return;
    }
    let got = exchange(port, &reset(stream));
    PACE.lock().asked(port, from, mk_uptime_ms(), got.is_ok());
    if !matches!(got, Err(Fault::Unanswered)) {
        reset_done((port, stream));
    }
}

/// One exchange with the proxy, when the pace allows one: the frame asked
/// and not answered, else the next bytes waiting, else a poll. A call that
/// goes unanswered is not an error; the frame stays asked for a later tick.
/// Only a call refused outright is, and it ends the conversation.
pub(super) fn ask_once(handle: u32) -> Result<(), ()> {
    let (port, frame) = with(handle, |s| match s.conv.over() {
        false => Some((s.port, s.conv.frame())),
        true => None,
    })?
    .ok_or(())?;
    let from = mk_uptime_ms();
    if !PACE.lock().may_ask(port, from) {
        return Ok(());
    }
    let got = exchange(port, &frame);
    PACE.lock().asked(port, from, mk_uptime_ms(), got.is_ok());
    let answered = with(handle, |s| match got {
        Ok(reply) => s.conv.answered(&reply),
        Err(Fault::Unanswered) => true,
        Err(Fault::Refused) => {
            s.conv.refused();
            false
        }
    })?;
    answered.then_some(()).ok_or(())
}

/// Take bytes to write to the proxy, and make one exchange. The bytes are
/// taken whether or not the proxy answered in time: the frame they went in
/// stays asked until it does (`sending`).
pub fn send(handle: u32, payload: &[u8]) -> Result<(), ()> {
    with(handle, |s| s.conv.take(payload).map_err(|_| ()))??;
    ask_once(handle)
}

/// Whether bytes written on `handle` are still waiting for the proxy's
/// answer, after asking it once more. An error when the proxy refused the
/// call outright, so the bytes will never be answered.
pub fn sending(handle: u32) -> Result<bool, ()> {
    if !with(handle, |s| s.conv.sending())? {
        return Ok(false);
    }
    ask_once(handle)?;
    with(handle, |s| s.conv.sending())
}

/// Take what the proxy has already answered.
///
/// An empty exchange is how the proxy is asked whether more has arrived. A
/// reply crosses several hops with a delay chosen at each one, so it is
/// almost never ready inside the same call that sent the request. Reading
/// only what a send happened to bring back meant the answer to every request
/// arrived after the only chance to collect it. While a frame is asked and
/// not answered, it is that frame that is asked again (`conv`).
///
/// `Closed` once the proxy has said the far end finished and every byte it
/// sent has been read, or the proxy can no longer be asked, or the handle
/// names no conversation: a reader then knows nothing more is coming rather
/// than waiting out its deadline for it.
pub fn recv(handle: u32, out: &mut [u8]) -> Recv {
    if with(handle, |s| !s.conv.holding() && !s.conv.over()) == Ok(true) {
        let _ = ask_once(handle);
    }
    let read = with(handle, |s| match s.conv.ended() {
        true => Recv::Closed,
        false => match s.conv.read(out) {
            0 => Recv::Empty,
            n => Recv::Bytes(n),
        },
    });
    read.unwrap_or(Recv::Closed)
}

/// Why the proxy can be asked nothing more on `handle`, when that is not the
/// far end finishing: a read there says `Closed` all the same, and the fetch
/// asks this to say what really happened.
pub fn broke(handle: u32) -> Option<Broke> {
    with(handle, |s| s.conv.why_broken()).ok().flatten()
}

/// The readiness bits a socket poll gives for `handle`, from what is held
/// and without asking the proxy: bit0 when a read would return bytes or the
/// close, bit1 while bytes may still be written. A pool asks this of a kept
/// conversation before it reuses it.
pub fn poll(handle: u32) -> Result<u8, ()> {
    with(handle, |s| {
        let readable = s.conv.holding() || s.conv.ended();
        let writable = !s.conv.over() && !s.conv.sending();
        u8::from(readable) | (u8::from(writable) << 1)
    })
}
