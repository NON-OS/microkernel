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

//! `AUTH_OK`: the session id, the idle timeout and the names given.

use nsh_wire::{AuthOk, WireError, MAX_IDLE_S};

pub fn list(names: &[&[u8]]) -> Vec<u8> {
    let mut out = vec![names.len() as u8];
    names.iter().for_each(|n| out.extend([&[n.len() as u8][..], n].concat()));
    out
}

#[test]
fn an_auth_ok_round_trips() {
    let mut out = [0u8; 64];
    let n = AuthOk::encode(&[7; 16], 900, &[b"CoreExec", b"IPC"], &mut out).expect("fits");
    let ok = AuthOk::parse(&out[..n]).expect("valid");
    assert_eq!((ok.session_id, ok.idle_s, ok.caps.len()), (&[7; 16], 900, 2));
    assert_eq!(ok.caps.iter().collect::<Vec<_>>(), [&b"CoreExec"[..], b"IPC"]);
}

#[test]
fn the_idle_timeout_is_a_second_to_a_day() {
    let mut out = [0u8; 64];
    for (idle, ok) in [(0, false), (1, true), (MAX_IDLE_S, true), (MAX_IDLE_S + 1, false)] {
        assert_eq!(AuthOk::encode(&[7; 16], idle, &[b"IPC"], &mut out).is_ok(), ok, "{idle}");
        let mut body = vec![7u8; 16];
        body.extend(idle.to_le_bytes());
        body.extend(list(&[b"IPC"]));
        assert_eq!(AuthOk::parse(&body).is_ok(), ok, "{idle}");
    }
}

#[test]
fn the_encoder_sends_no_list_a_receiver_would_refuse() {
    let mut out = [0u8; 64];
    assert_eq!(AuthOk::encode(&[7; 16], 9, &[b"IPC", b"IPC"], &mut out), Err(WireError::Value));
    assert_eq!(AuthOk::encode(&[7; 16], 9, &[b"ipc"], &mut out), Err(WireError::Value));
}
