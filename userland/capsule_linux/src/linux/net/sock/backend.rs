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

//! What carries a stream that reaches outside the family: a net.sockets
//! handle (the Nym mixnet), or a net.anon stream (the Anyone network), with
//! the bytes taken from net.anon that the guest has not read yet.

use alloc::vec::Vec;

pub enum Backend {
    /// A net.sockets mixnet socket.
    Sockets(u32),
    /// A stream net.anon opened for this capsule.
    Anon(Anon),
}

/// Which service a call on the stream goes to, without the stream's state.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Via {
    Sockets(u32),
    Anon,
}

/// A net.anon stream to close: the service's port and the stream's id.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Close {
    pub port: u32,
    pub id: u16,
}

/// How the far end finished, as a read reports it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum End {
    /// It finished cleanly: end of file.
    Eof,
    /// This errno, reported once, then end of file.
    Error(i64),
}

pub struct Anon {
    /// net.anon's service port, as the route named it at connect.
    pub port: u32,
    /// The stream's id there, until it is closed: taken by the first close,
    /// so no second one is ever sent.
    pub id: Option<u16>,
    /// What a read from net.anon brought and the guest has not read. A read
    /// from net.anon is made only while this is empty, and one brings at
    /// most PAYLOAD_MAX, so it never holds more (`fill`).
    pub rx: Vec<u8>,
    /// Set once a read has seen the far end finish.
    pub end: Option<End>,
    /// Some byte has arrived: a stream ended after that was reset, one that
    /// ended before it was refused.
    pub heard: bool,
}

impl Anon {
    pub fn new(port: u32, id: u16) -> Anon {
        Anon { port, id: Some(id), rx: Vec::new(), end: None, heard: false }
    }

    /// The close to send, the first time this is asked; None after.
    pub fn close(&mut self) -> Option<Close> {
        let port = self.port;
        self.id.take().map(|id| Close { port, id })
    }

    /// True when a read from net.anon may be made: nothing is held, the far
    /// end has not finished, and the stream is still open there.
    pub fn wants_fill(&self) -> bool {
        self.rx.is_empty() && self.end.is_none() && self.id.is_some()
    }
}

impl Backend {
    pub fn via(&self) -> Via {
        match self {
            Backend::Sockets(h) => Via::Sockets(*h),
            Backend::Anon(_) => Via::Anon,
        }
    }

    pub fn anon(&self) -> Option<&Anon> {
        match self {
            Backend::Anon(a) => Some(a),
            Backend::Sockets(_) => None,
        }
    }

    pub fn anon_mut(&mut self) -> Option<&mut Anon> {
        match self {
            Backend::Anon(a) => Some(a),
            Backend::Sockets(_) => None,
        }
    }
}
