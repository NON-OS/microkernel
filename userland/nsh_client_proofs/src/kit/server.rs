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

//! The test server, made only from nsh_wire and nsh_crypto. Proofs drive
//! it a frame at a time, so each can change one thing about a good run.

use super::rng::Det;
use nsh_crypto::{encapsulate, handshake_keys, Keys, Opener, Sealer, Secret, Transcript};
use nsh_wire::{len_field, ClientHello, Expect, Kind, ServerHello, SERVER_HELLO_LEN, TAG_LEN};

pub struct Server {
    pub th: Transcript,
    pub th_hello: [u8; 64],
    pub(super) sealer: Option<Sealer>,
    pub(super) opener: Option<Opener>,
    pub(super) hs: Option<Secret<64>>,
}

impl Default for Server {
    fn default() -> Self {
        let (th, th_hello) = (Transcript::default(), [0; 64]);
        Server { th, th_hello, sealer: None, opener: None, hs: None }
    }
}

impl Server {
    /// Read `CLIENT_HELLO`, make keys, and give the `SERVER_HELLO` frame.
    /// `edit` changes the hello in flight, after the server has used it.
    pub fn hello(&mut self, wire: &[u8], edit: impl FnOnce(&mut Vec<u8>)) -> Vec<u8> {
        let body = &wire[5..];
        assert_eq!(wire[4], Kind::ClientHello.byte());
        let ch = ClientHello::parse(body).expect("client hello");
        let (ct, ss) = encapsulate(ch.kem_ek, Secret::new(Det::new(0x22).bytes())).unwrap();
        let mut sh = vec![0u8; SERVER_HELLO_LEN];
        ServerHello { random: &[0x53; 32], kem_ct: &ct }.encode(&mut sh).unwrap();
        self.th.push(Kind::ClientHello, body);
        self.th.push(Kind::ServerHello, &sh);
        self.th_hello = self.th.digest();
        let (Keys { c2s, s2c }, hs) = handshake_keys(ss, &self.th_hello);
        (self.sealer, self.opener, self.hs) =
            (Some(Sealer::new(s2c)), Some(Opener::new(c2s)), Some(hs));
        let mut frame = vec![Kind::ServerHello.byte()];
        frame.extend_from_slice(&sh);
        edit(&mut frame);
        let mut out = len_field(frame.len() - 1, Expect::ServerHello).unwrap().to_vec();
        out.extend_from_slice(&frame);
        out
    }

    /// Seal a type byte and body, whatever they are, with a length field
    /// that matches them.
    pub fn seal_inner(&mut self, inner: &[u8]) -> Vec<u8> {
        let len = u32::try_from(inner.len() + TAG_LEN).unwrap().to_le_bytes();
        let mut out = vec![0u8; inner.len() + TAG_LEN];
        self.sealer.as_mut().unwrap().seal(len, inner, &mut out).unwrap();
        [&len[..], &out].concat()
    }

    pub fn seal(&mut self, kind: Kind, body: &[u8]) -> Vec<u8> {
        self.seal_inner(&[&[kind.byte()][..], body].concat())
    }
}
