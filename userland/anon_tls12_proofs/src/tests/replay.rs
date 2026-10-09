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


//! Running the client over recorded bytes, and recording them.
//!
//! A handshake is deterministic once randomness is seeded: the client random
//! and the ECDHE scalar are the only draws. So the bytes a real server sent
//! one client can be fed to the same client again, and the client must write
//! exactly what it wrote the first time and reach the same end.

use std::string::String;
use std::vec::Vec;

use nonos_tls::{Io, SessionError};

use crate::tls12::{connect, Tls12Error};

/// The seed every recording and replay starts from.
pub const SEED: u64 = 0x005E_ED0F_7153_0012;
/// The name the client sends, as net.anon sends a relay's address.
pub const SNI: &[u8] = b"10.1.0.1";
/// What the client writes once the session is up.
pub const PING: &[u8] = b"ping from nonos\n";

/// How a run ended: the handshake's error, or the plaintext the server sent
/// and whether it then ended the session.
#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Refused(Tls12Error),
    Session { received: Vec<u8>, done: bool },
}

impl Outcome {
    pub fn line(&self) -> String {
        match self {
            Outcome::Refused(e) => std::format!("refused {e:?}"),
            Outcome::Session { received, done } => {
                std::format!("session {} done={done}", String::from_utf8_lossy(received).escape_default())
            }
        }
    }
}

/// Drive one run over `io`: handshake, the ping, then read until the server
/// ends the session or `quiet` says it has gone quiet.
pub fn run<S: Io>(io: &mut S, mut quiet: impl FnMut() -> bool) -> Outcome {
    nonos_libc::seed(SEED);
    let mut stream = match connect(io, SNI) {
        Ok(s) => s,
        Err(e) => return Outcome::Refused(e),
    };
    stream.write_all(io, PING).expect("the ping goes");
    let mut received = Vec::new();
    loop {
        match stream.read(io) {
            Ok(bytes) if !bytes.is_empty() => received.extend_from_slice(&bytes),
            Ok(_) => {}
            Err(e) => return Outcome::Refused(e),
        }
        if stream.is_done() || quiet() {
            return Outcome::Session { received, done: stream.is_done() };
        }
    }
}

/// Recorded server bytes, served in chunks of `chunk`, with every client
/// write checked against what the client wrote when they were recorded.
pub struct Replay {
    server: Vec<u8>,
    at: usize,
    chunk: usize,
    client: Vec<u8>,
    written: usize,
    /// Do not compare the client's writes: for tampered server bytes, where
    /// the client rightly writes something else or stops early.
    lenient: bool,
    pub dry: usize,
}

impl Replay {
    pub fn new(server: &[u8], client: &[u8], chunk: usize) -> Self {
        Self { server: server.to_vec(), at: 0, chunk, client: client.to_vec(), written: 0, lenient: false, dry: 0 }
    }

    pub fn lenient(server: &[u8]) -> Self {
        Self { server: server.to_vec(), at: 0, chunk: usize::MAX, client: Vec::new(), written: 0, lenient: true, dry: 0 }
    }

    /// Whether the client wrote all it wrote when recorded, and no more.
    pub fn wrote_all(&self) -> bool {
        self.written == self.client.len()
    }
}

impl Io for Replay {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        if self.lenient {
            return Ok(());
        }
        let end = self.written + data.len();
        assert!(end <= self.client.len(), "the client wrote more than it did when recorded");
        assert_eq!(&self.client[self.written..end], data, "the client wrote other bytes than when recorded, at {}", self.written);
        self.written = end;
        Ok(())
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        if self.at == self.server.len() {
            // Run dry: let the clock reach the handshake's quiet deadline.
            self.dry += 1;
            nonos_libc::advance(1000);
            return Ok(0);
        }
        let n = self.chunk.min(into.len()).min(self.server.len() - self.at);
        into[..n].copy_from_slice(&self.server[self.at..self.at + n]);
        self.at += n;
        Ok(n)
    }
}
