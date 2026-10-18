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

//! The soundness check has teeth: each way a responder could stray — a
//! reply past the MTU, the wrong identifier, a channel-opening code, a
//! reject without its reason, or timing accepted with no request to accept
//! — is caught by the same predicate the sweep leans on.

use super::signal_tests::{sound_answer, LE_SIGNAL_MTU};
use nonos_bt_wire::l2cap::conn_param::ConnParams;
use nonos_bt_wire::l2cap::signal::*;
use nonos_bt_wire::l2cap::{signal, Answer};

fn with_reply(code: u8, id: u8, data: &[u8]) -> Answer {
    Answer { reply: signal(code, id, data).ok(), update: None }
}

#[test]
fn an_unsound_answer_is_caught() {
    let reason = &NOT_UNDERSTOOD.to_le_bytes();
    // A genuine reject for the request's identifier passes.
    assert!(sound_answer(LE_CREDIT_CONNECTION_REQUEST, 7, &with_reply(COMMAND_REJECT, 7, reason)));
    // The wrong identifier.
    assert!(!sound_answer(COMMAND_REJECT, 7, &with_reply(COMMAND_REJECT, 8, reason)));
    // A code that would open a channel.
    assert!(!sound_answer(
        LE_CREDIT_CONNECTION_REQUEST,
        7,
        &with_reply(LE_CREDIT_CONNECTION_REQUEST, 7, &[0; 4])
    ));
    // A reject carrying something other than a two-byte reason.
    assert!(!sound_answer(COMMAND_REJECT, 7, &with_reply(COMMAND_REJECT, 7, &[0; 4])));
    // An update response longer than the signaling MTU.
    assert!(!sound_answer(
        COMMAND_REJECT,
        7,
        &with_reply(CONN_PARAM_UPDATE_RESPONSE, 7, &[0; LE_SIGNAL_MTU])
    ));
    // Timing accepted with no update request to accept it.
    let p = ConnParams { interval_min: 24, interval_max: 40, latency: 0, timeout: 200 };
    let phantom = Answer {
        reply: signal(CONN_PARAM_UPDATE_RESPONSE, 7, &0u16.to_le_bytes()).ok(),
        update: Some(p),
    };
    assert!(!sound_answer(COMMAND_REJECT, 7, &phantom));
}
