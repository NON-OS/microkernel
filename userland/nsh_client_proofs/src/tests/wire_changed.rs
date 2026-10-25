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

//! A byte changed on the wire in each sealed frame the server sends after
//! `HOST_PROOF`: `AUTH_OK`, `AUTH_REFUSED`, and every run frame. None
//! opens, none is answered, and the session is over.

use super::check::logged_in;
use super::silent;
use crate::kit::pair::Pair;
use crate::kit::server::Server;
use nsh_client::Refusal;
use nsh_wire::Kind;

fn flipped(mut f: Vec<u8>, at: usize) -> Vec<u8> {
    let i = at.min(f.len() - 1);
    f[i] ^= 0x80;
    f
}

#[test]
fn a_changed_auth_answer_does_not_open() {
    for at in [4usize, 5, 30, usize::MAX] {
        let mut p = Pair::good();
        p.to_auth();
        let f = flipped(p.server.auth_ok(&Server::auth_ok_body()), at);
        assert_eq!(p.feed(&f), Err(Refusal::Open), "AUTH_OK byte {at}");
        silent(&mut p);
        let mut p = Pair::good();
        p.to_auth();
        let f = flipped(p.server.seal(Kind::AuthRefused, &24u16.to_le_bytes()), at);
        assert_eq!(p.feed(&f), Err(Refusal::Open), "AUTH_REFUSED byte {at}");
        silent(&mut p);
    }
}

#[test]
fn a_changed_session_frame_does_not_open() {
    let frames: [(Kind, &[u8]); 6] = [
        (Kind::Output, b"bin etc\n"),
        (Kind::Exit, &[0, 0, 0, 0]),
        (Kind::Record, &[1, b'N']),
        (Kind::Close, &[]),
        (Kind::Abort, &[1, 0]),
        (Kind::EscalateResult, &[33, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    ];
    for (kind, body) in frames {
        for at in [4usize, 5, usize::MAX] {
            let mut p = logged_in();
            let f = flipped(p.server.seal(kind, body), at);
            assert_eq!(p.feed(&f), Err(Refusal::Open), "{kind:?} byte {at}");
            silent(&mut p);
        }
    }
}
