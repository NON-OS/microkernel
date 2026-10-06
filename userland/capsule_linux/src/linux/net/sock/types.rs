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

//! One socket, as the family sees it.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use super::backend::Backend;
use super::name::{Gram, UName};
use super::opts::Opts;

pub use super::kinds::{Addr, Domain, Proto};

pub struct Sock {
    pub domain: Domain,
    pub proto: Proto,
    pub local: Option<Addr>,
    pub remote: Option<Addr>,
    /// Set by listen, with the queue of connections accept has not taken.
    pub listening: bool,
    pub backlog: usize,
    pub pending: VecDeque<u32>,
    /// Connects the full queue turned away, oldest first (`syn`).
    pub syn: VecDeque<u32>,
    /// This end's connect waits in a listener's `syn` queue.
    pub connecting: bool,
    /// The other end of a stream, until it is let go.
    pub peer: Option<u32>,
    pub connected: bool,
    /// Bytes the peer wrote that this end has not read.
    pub rx: VecDeque<u8>,
    /// Datagrams waiting to be read.
    pub grams: VecDeque<Gram>,
    /// The peer will write nothing more: it shut its side or it is gone.
    pub eof: bool,
    pub wr_shut: bool,
    pub rd_shut: bool,
    /// Written to after the peer was gone: every later write is EPIPE.
    pub broken: bool,
    /// SO_ERROR: reported once, by the next call that looks.
    pub error: i64,
    pub opts: Opts,
    /// What carries a stream that reaches outside the family: a net.sockets
    /// handle (the Nym mixnet) or a net.anon stream (the Anyone network).
    pub svc: Option<Backend>,
    /// The processes that hold a descriptor naming this socket.
    pub holders: Vec<u32>,
    /// A Unix socket's own name, and its connected peer's.
    pub uname: Option<UName>,
    pub upeer: Option<UName>,
}
