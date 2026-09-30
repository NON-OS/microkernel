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

//! What the fetch machine asks of the network, as a trait it is handed.
//!
//! The machine decides; whatever implements this carries it out: net.sockets
//! in the capsule, a scripted socket in the host proofs.

use crate::browser::net::Source;

/// The poll bit for a socket a read would return data on.
pub const READABLE: u8 = 1;
/// The poll bit for a socket a send would be taken on. For a connection
/// being made, that is the moment its handshake has finished.
pub const WRITABLE: u8 = 2;

/// What a name resolved to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Resolved {
    Ip([u8; 4]),
    /// The resolver answered that the name has no address.
    Unknown,
    /// The resolver could not be asked, or did not answer in time.
    Unavailable,
}

/// A socket call the network service refused or could not carry out.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Refused;

pub trait Wire: Source {
    /// The wall clock certificates are judged against, as YYYYMMDDhhmmss.
    fn rtc_now(&self) -> u64;
    /// Whether every connection rides the one mixnet conversation.
    fn mixnet(&self) -> bool;
    fn open(&mut self) -> Result<u32, Refused>;
    /// An address already known for `host`, found without asking anyone.
    fn cached(&self, host: &str) -> Option<[u8; 4]>;
    /// Ask the resolver; this may wait for its answer.
    fn resolve(&mut self, host: &str) -> Resolved;
    /// Start connecting and return at once.
    fn connect_nb(&mut self, handle: u32, ip: [u8; 4], port: u16) -> Result<(), Refused>;
    /// Resolve and connect in one call that waits for the handshake.
    fn connect_host(&mut self, handle: u32, host: &str, port: u16) -> Result<(), Refused>;
    fn poll(&mut self, handle: u32) -> Result<u8, Refused>;
    fn send(&mut self, handle: u32, bytes: &[u8]) -> Result<(), Refused>;
    fn close(&mut self, handle: u32);
    /// One line for the debug console.
    fn trace(&mut self, line: &[u8]);
}
