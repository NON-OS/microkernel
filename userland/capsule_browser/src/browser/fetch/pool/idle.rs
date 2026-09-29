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

//! A connection between requests.

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::fetch::keep::KeptConn;
use crate::browser::fetch::types::TlsCtx;
use crate::browser::url::{Scheme, Url};

pub struct Idle {
    pub host: String,
    pub port: u16,
    pub https: bool,
    pub handle: u32,
    pub tls: Option<TlsCtx>,
    /* TLS only: ciphertext not yet let go of, and plaintext already used. */
    pub buf: Vec<u8>,
    pub consumed: usize,
    pub tx_seq: u64,
    pub used: u8,
    pub since_ms: i64,
}

impl Idle {
    pub fn kept(k: KeptConn, now: i64) -> Idle {
        let (host, port, handle, tls) = (k.host, k.port, k.handle, Some(k.tls));
        let (buf, consumed, tx_seq, used) = (k.buf, k.consumed, k.tx_seq, k.used);
        Idle { host, port, https: true, handle, tls, buf, consumed, tx_seq, used, since_ms: now }
    }

    pub fn serves(&self, url: &Url) -> bool {
        self.host == url.host
            && self.port == url.port
            && self.https == (url.scheme == Scheme::Https)
    }
}
