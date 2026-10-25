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

//! The two plaintext hellos. Each is a fixed preamble (magic, version,
//! suite, flags), a 32-byte random, and the X-Wing key or ciphertext. The
//! preamble is judged first, so a peer speaking another version is told
//! `VERSION`, not `PROTOCOL`, whatever size its hello is; the rest must
//! then be exactly the random and the key, nothing short and nothing more.

use crate::hello_preamble::{open, write};
use crate::WireError;

pub const MAGIC: [u8; 4] = *b"NSH1";
pub const VERSION: u16 = 1;
pub const SUITE: u16 = 1;
pub const RANDOM_LEN: usize = 32;
pub const XWING_EK_LEN: usize = 1216;
pub const XWING_CT_LEN: usize = 1120;
const PREAMBLE: usize = 10;
pub const CLIENT_HELLO_LEN: usize = PREAMBLE + RANDOM_LEN + XWING_EK_LEN;
pub const SERVER_HELLO_LEN: usize = PREAMBLE + RANDOM_LEN + XWING_CT_LEN;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientHello<'a> {
    pub random: &'a [u8; RANDOM_LEN],
    pub kem_ek: &'a [u8; XWING_EK_LEN],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerHello<'a> {
    pub random: &'a [u8; RANDOM_LEN],
    pub kem_ct: &'a [u8; XWING_CT_LEN],
}

impl<'a> ClientHello<'a> {
    pub fn parse(body: &'a [u8]) -> Result<Self, WireError> {
        let mut r = open(body)?;
        let hello = ClientHello { random: r.array()?, kem_ek: r.array()? };
        r.finish().map(|_| hello)
    }

    pub fn encode(&self, out: &mut [u8]) -> Result<usize, WireError> {
        write(out, self.random, self.kem_ek)
    }
}

impl<'a> ServerHello<'a> {
    pub fn parse(body: &'a [u8]) -> Result<Self, WireError> {
        let mut r = open(body)?;
        let hello = ServerHello { random: r.array()?, kem_ct: r.array()? };
        r.finish().map(|_| hello)
    }

    pub fn encode(&self, out: &mut [u8]) -> Result<usize, WireError> {
        write(out, self.random, self.kem_ct)
    }
}
