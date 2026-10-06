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


//! net.anon's TLS 1.2 client against a "server" that sends these bytes. The
//! handshake runs with the replay seed, so a recorded OpenSSL flight in the
//! corpus completes and mutations of it reach every later step. Nothing may
//! panic, and each message reader is driven on its own as well.

#![no_main]

use anon_tls12_proofs::tls12::record::take;
use anon_tls12_proofs::tls12::server::{certificate, certificate_request, key_exchange, server_hello};
use anon_tls12_proofs::tls12::spki::spki;
use anon_tls12_proofs::tls12::connect;
use libfuzzer_sys::fuzz_target;
use nonos_tls::{Io, SessionError};

struct Server<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Io for Server<'_> {
    fn write_all(&mut self, _data: &[u8]) -> Result<(), SessionError> {
        Ok(())
    }
    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        if self.at >= self.bytes.len() {
            // Run dry: move the clock past the quiet deadline at once.
            nonos_libc::advance(10_000);
            return Ok(0);
        }
        let n = into.len().min(self.bytes.len() - self.at).min(1 + self.at % 700);
        into[..n].copy_from_slice(&self.bytes[self.at..self.at + n]);
        self.at += n;
        Ok(n)
    }
}

fuzz_target!(|data: &[u8]| {
    nonos_libc::seed(0x005E_ED0F_7153_0012);
    let mut server = Server { bytes: data, at: 0 };
    if let Ok(mut stream) = connect(&mut server, b"10.1.0.1") {
        let _ = stream.write_all(&mut server, b"ping");
        let _ = stream.read(&mut server);
    }
    let _ = server_hello(data);
    let _ = certificate(data);
    let _ = key_exchange(data);
    let _ = certificate_request(data);
    let _ = spki(data);
    let mut buf = data.to_vec();
    while let Ok(Some(_)) = take(&mut buf, true) {}
});
