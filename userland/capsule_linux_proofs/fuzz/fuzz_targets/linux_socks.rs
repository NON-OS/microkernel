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

//! Any bytes a guest writes to the SOCKS endpoint at 127.0.0.1:9050, in the
//! pieces the input names (nonos_capsule_linux_proofs::linux::net::routed::socks):
//! the SOCKS5 greeting, an RFC 1929 username and password, SOCKS4 and SOCKS4a.
//! No piece is taken past what was written or past REQUEST_MAX, nothing is
//! heard after the conversation ends, the guest is told only a few short
//! replies, and a CONNECT names an address or a name the endpoint carries.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_capsule_linux_proofs::linux::net::routed::socks::kinds::{name_ok, REQUEST_MAX};
use nonos_capsule_linux_proofs::linux::net::routed::socks::{sign_in, v4, v5, Heard, Talk, To};

/// The most the endpoint says before a CONNECT: a method choice, a sign-in
/// status and a refusal, with room to spare.
const SAID_MAX: usize = 64;

fuzz_target!(|data: &[u8]| {
    let Some((&cut, bytes)) = data.split_first() else {
        return;
    };
    let _ = (v4::request(bytes), v5::greeting(bytes), v5::request(bytes));
    let _ = sign_in::credentials(bytes);
    let (mut t, mut said, mut at) = (Talk::default(), Vec::new(), 0);
    let piece_len = 1 + usize::from(cut);
    while at < bytes.len() {
        let piece = &bytes[at..at.saturating_add(piece_len).min(bytes.len())];
        let (took, heard) = t.hear(piece, &mut said);
        assert!(took <= piece.len() && took <= REQUEST_MAX, "taken past what was written");
        assert!(said.len() <= SAID_MAX, "the guest was told more than a few replies");
        at += took;
        match heard {
            Heard::More if took > 0 => {}
            Heard::Connect(_, c) => {
                assert!(matches!(&c.to, To::V4(_)) || matches!(&c.to, To::Name(n) if name_ok(n)));
                return;
            }
            _ => return,
        }
    }
});
