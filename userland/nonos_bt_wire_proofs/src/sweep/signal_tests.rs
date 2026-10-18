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

//! E-66: the LE signaling responder under a hostile peer (Vol 3, Part A,
//! 4). Seeded codes, identifiers and lengths over `answer_central`: it
//! never panics, every reply fits the signaling MTU and parses as a reject
//! or an update response for the request's own identifier, and accepted
//! timing — the one thing it "opens" — follows only an accepted request.

use super::rng::each;
use nonos_bt_wire::l2cap::conn_param::UPDATE_ACCEPTED;
use nonos_bt_wire::l2cap::signal::*;
use nonos_bt_wire::l2cap::{answer_central, parse_signal, signal, Answer};

/// The LE signaling channel MTU (Vol 3, Part A, 4): a host never puts a
/// larger PDU on the wire.
pub(super) const LE_SIGNAL_MTU: usize = 23;

/// What the answer to a request `(code, id)` must satisfy: the reply, if
/// any, fits the MTU and parses as a reject or an update response for that
/// identifier; accepted timing follows only an accepted update request.
pub(super) fn sound_answer(code: u8, id: u8, a: &Answer) -> bool {
    let reply_ok = match &a.reply {
        None => true,
        Some(r) => parse_signal(r).is_ok_and(|s| {
            r.len() <= LE_SIGNAL_MTU
                && s.id == id
                && (s.code == COMMAND_REJECT || s.code == CONN_PARAM_UPDATE_RESPONSE)
                && (s.code != COMMAND_REJECT || s.data.len() == 2)
        }),
    };
    let update_ok = a.update.is_none()
        || (code == CONN_PARAM_UPDATE_REQUEST
            && a.reply.as_ref().is_some_and(|r| {
                parse_signal(r).is_ok_and(|s| s.data == UPDATE_ACCEPTED.to_le_bytes())
            }));
    reply_ok && update_ok
}

#[test]
fn answers_on_the_signaling_channel_are_sound() {
    let codes = [
        COMMAND_REJECT,
        DISCONNECTION_REQUEST,
        CONN_PARAM_UPDATE_REQUEST,
        CONN_PARAM_UPDATE_RESPONSE,
        LE_CREDIT_CONNECTION_REQUEST,
    ];
    each(|rng| {
        let code = if rng.below(2) == 0 { rng.pick(&codes) } else { rng.byte() };
        let (id, data) = (rng.byte(), rng.span(0, 12));
        let frame = match signal(code, id, &data) {
            Ok(f) if rng.below(4) != 0 => f,
            _ => [&[code, id][..], &data].concat(),
        };
        if let Ok(a) = answer_central(&frame) {
            let s = parse_signal(&frame).expect("Ok only follows a clean parse");
            assert!(sound_answer(s.code, s.id, &a), "unsound answer to {frame:?}");
        }
    });
}
