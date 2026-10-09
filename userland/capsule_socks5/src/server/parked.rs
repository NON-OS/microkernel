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

//! What net.socks5 answers before net.nym is up.
//!
//! It used to answer every frame with a bare close. The browser read that as
//! the exit hanging up ("the exit closed the connection") and the terminal as
//! the mixnet ending the connection: both blamed something that had not
//! been reached. Now the frames get the answers a SOCKS server with no
//! network gives: a greeting is accepted, a CONNECT is refused with "network
//! unreachable" (3), which every client of this proxy reads as "not connected
//! yet" and asks again after a pause, and a status ask says net.socks5 is
//! waiting for net.nym. Nothing is kept: every answer follows from the frame
//! alone, so the same frame asked again gets the same answer.
//!
//! Pure; capsule_socks5_proofs holds it.

extern crate alloc;

use alloc::vec::Vec;

use super::reply::{progress, STEP_WAITING_FOR_NYM, STREAM_CLOSED, STREAM_OPEN};
use super::request::{ask, Ask};
use crate::wire::{
    method_reply, offers_no_auth, reply, CMD_CONNECT, REPLY_LEN, REP_NET_UNREACH, VER,
};

/// The answer to `frame` while the mixnet transport is not there yet.
pub fn parked(frame: &[u8]) -> Vec<u8> {
    match ask(frame) {
        Some(Ask::Status) => progress(false, STEP_WAITING_FOR_NYM, 0),
        Some(Ask::Reset | Ask::ResetOn(_)) => Vec::from([STREAM_OPEN]),
        Some(Ask::Stream(body) | Ask::Numbered(_, body) | Ask::NumberedOn(_, _, body)) => {
            socks(body)
        }
        None => Vec::from([STREAM_CLOSED]),
    }
}

/// A conversation's bytes: nothing yet, a greeting, or a CONNECT.
fn socks(body: &[u8]) -> Vec<u8> {
    if body.is_empty() {
        return Vec::from([STREAM_OPEN]);
    }
    /* A greeting is exactly as long as the methods it counts; a CONNECT is
     * longer than any greeting offering one method, and names the command
     * after the version. */
    let greeting = body.len() >= 2 && body[0] == VER && body.len() == 2 + body[1] as usize;
    if greeting {
        return match offers_no_auth(body) {
            Some(true) => [&[STREAM_OPEN][..], &method_reply(true)].concat(),
            _ => [&[STREAM_CLOSED][..], &method_reply(false)].concat(),
        };
    }
    if body.len() >= 4 && body[0] == VER && body[1] == CMD_CONNECT {
        let mut out = [0u8; REPLY_LEN];
        let len = reply(REP_NET_UNREACH, &mut out);
        return [&[STREAM_CLOSED][..], &out[..len]].concat();
    }
    Vec::from([STREAM_CLOSED])
}
