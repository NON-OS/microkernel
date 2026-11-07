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

//! A client built from nsh_wire and nsh_crypto alone, which sends whatever
//! a proof asks: frames out of order, of unknown types, with broken bodies
//! or changed bytes. It keeps the transcript and both directions' keys as
//! the real client does, so everything it sends is otherwise well formed.

use super::rng::Det;
use nsh_crypto::{handshake_keys, ClientKem, Keys, Opener, Sealer, Secret, Transcript};
use nsh_wire::{len_field, ClientHello, Expect, Kind, ServerHello, CLIENT_HELLO_LEN};

pub struct Raw {
    pub th: Transcript,
    pub(super) kem: Option<ClientKem>,
    pub(super) hs: Option<Secret<64>>,
    pub(super) sealer: Option<Sealer>,
    pub(super) opener: Option<Opener>,
    /// The client's hello, whole.
    pub hello: Vec<u8>,
}

impl Default for Raw {
    fn default() -> Raw {
        Raw::new()
    }
}

impl Raw {
    pub fn new() -> Raw {
        let mut rng = Det::new(0xA1);
        let (kem, ek) = ClientKem::new(Secret::new(rng.bytes()));
        let mut body = [0u8; CLIENT_HELLO_LEN];
        let n = ClientHello { random: &rng.bytes(), kem_ek: &ek }.encode(&mut body).expect("hello");
        let mut th = Transcript::default();
        th.push(Kind::ClientHello, &body[..n]);
        let mut hello = len_field(n, Expect::ClientHello).expect("len").to_vec();
        hello.push(Kind::ClientHello.byte());
        hello.extend_from_slice(&body[..n]);
        Raw { th, kem: Some(kem), hs: None, sealer: None, opener: None, hello }
    }

    /// Take `SERVER_HELLO` and `HOST_PROOF` from the server's first bytes.
    pub fn handshake(&mut self, wire: &[u8]) -> Vec<(u8, Vec<u8>)> {
        let n = u32::from_le_bytes(wire[..4].try_into().expect("len")) as usize;
        assert_eq!(wire[4], Kind::ServerHello.byte());
        let body = &wire[5..4 + n];
        let sh = ServerHello::parse(body).expect("server hello");
        let ss = self.kem.take().expect("one hello").decapsulate(sh.kem_ct);
        self.th.push(Kind::ServerHello, body);
        let (Keys { c2s, s2c }, hs) = handshake_keys(ss, &self.th.digest());
        (self.sealer, self.opener, self.hs) =
            (Some(Sealer::new(c2s)), Some(Opener::new(s2c)), Some(hs));
        let frames = self.open(&wire[4 + n..]);
        if let Some((_, proof)) = frames.iter().find(|f| f.0 == Kind::HostProof.byte()) {
            self.th.push(Kind::HostProof, proof);
        }
        frames
    }
}
