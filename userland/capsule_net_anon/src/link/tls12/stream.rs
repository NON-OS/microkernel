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


//! An open TLS 1.2 session: application data both ways, with the four
//! calls the link layer makes of a TLS 1.3 stream.

extern crate alloc;

use alloc::vec::Vec;

use nonos_tls::Io;

use super::alert::{description, CLOSE_NOTIFY};
use super::constants::*;
use super::error::Tls12Error;
use super::record::{take, Direction};

pub struct Tls12Stream {
    write: Direction,
    read: Direction,
    partial: Vec<u8>,
    plain: Vec<u8>,
    leaf: Vec<u8>,
    done: bool,
}

impl Tls12Stream {
    pub(super) fn new(write: Direction, read: Direction, leftover: Vec<u8>, leaf: Vec<u8>) -> Self {
        Self { write, read, partial: leftover, plain: Vec::new(), leaf, done: false }
    }

    /// The relay's certificate, DER, for the CERTS cell to bind.
    pub fn leaf(&self) -> &[u8] {
        &self.leaf
    }

    /// True once the relay closed the session or broke it.
    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn write_all<S: Io>(&mut self, io: &mut S, body: &[u8]) -> Result<(), Tls12Error> {
        if self.done {
            return Err(Tls12Error::Io);
        }
        for chunk in body.chunks(PLAINTEXT_MAX) {
            let record = self.write.seal(CT_APPLICATION, chunk)?;
            io.write_all(&record).map_err(|_| Tls12Error::Io)?;
        }
        Ok(())
    }

    /// Plaintext that has arrived. Empty means nothing yet; `is_done` says
    /// whether more can come.
    pub fn read<S: Io>(&mut self, io: &mut S) -> Result<Vec<u8>, Tls12Error> {
        let mut chunk = [0u8; 4096];
        loop {
            while !self.done && self.absorb() {}
            if !self.plain.is_empty() || self.done {
                return Ok(core::mem::take(&mut self.plain));
            }
            let n = io.read(&mut chunk).map_err(|_| Tls12Error::Io)?;
            if n == 0 {
                return Ok(Vec::new());
            }
            if self.partial.len().saturating_add(n) > FLIGHT_MAX {
                return Err(Tls12Error::TooLarge);
            }
            self.partial.extend_from_slice(&chunk[..n.min(chunk.len())]);
        }
    }

    /// Take one whole record, if there is one. Application data is kept. A
    /// close_notify ends the session cleanly; any other alert, a record that
    /// does not authenticate or parse, a ChangeCipherSpec, and any handshake
    /// message end it as broken. A HelloRequest is a renegotiation, which
    /// this client never does.
    fn absorb(&mut self) -> bool {
        let (kind, body) = match take(&mut self.partial, false) {
            Ok(Some(record)) => record,
            Ok(None) => return false,
            Err(_) => {
                self.done = true;
                return false;
            }
        };
        let Ok(mut opened) = self.read.open(kind, &body) else {
            self.done = true;
            return false;
        };
        match kind {
            CT_APPLICATION => self.plain.extend_from_slice(&opened),
            CT_ALERT => {
                if description(&opened) == Ok(CLOSE_NOTIFY) {
                    crate::trace::say(b"tls12 relay closed the session");
                }
                self.done = true;
            }
            CT_HANDSHAKE if opened.first() == Some(&HS_HELLO_REQUEST) => {
                crate::trace::say(b"tls12 relay asked to renegotiate, link ended");
                self.done = true;
            }
            _ => self.done = true,
        }
        crate::crypto::wipe::wipe(&mut opened);
        !self.done
    }
}
