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

use crate::browser::net::mixnet::Way;
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
    /// The resolver answered that no DNS server it asks is reachable: every
    /// name fails the same way until the network comes back.
    NoDns,
}

/// A socket call the network service refused or could not carry out.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Refused;

/// Why a connect by name did not go through.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HostFail {
    /// The service refused it, or the connection was not made.
    Refused,
    /// net.sockets could not have the name looked up: no DNS server is
    /// reachable (its E_NO_DNS).
    NoDns,
}

pub trait Wire: Source {
    /// The wall clock certificates are judged against, as YYYYMMDDhhmmss.
    fn rtc_now(&self) -> u64;
    /// The wall clock in milliseconds, for the cookies a request carries.
    /// Every wait and deadline reads `now_ms`, the uptime clock, instead:
    /// the wall clock is set when NTP first answers, and on a machine
    /// whose clock was not already right that is a jump of hours or years
    /// in the middle of a load, which ran every fetch past its budget.
    fn wall_ms(&self) -> i64;
    /// The way a connection to `host` leaves now (`net::mixnet::way`).
    fn way(&self, host: &str) -> Way;
    /// A socket for a connection that leaves `way`.
    fn open(&mut self, way: Way) -> Result<u32, Refused>;
    /// An address already known for `host`, found without asking anyone.
    fn cached(&self, host: &str) -> Option<[u8; 4]>;
    /// Ask the resolver; this may wait for its answer.
    fn resolve(&mut self, host: &str) -> Resolved;
    /// Start connecting and return at once.
    fn connect_nb(&mut self, handle: u32, ip: [u8; 4], port: u16) -> Result<(), Refused>;
    /// Resolve and connect in one call that waits for the handshake.
    fn connect_host(&mut self, handle: u32, host: &str, port: u16) -> Result<(), HostFail>;
    fn poll(&mut self, handle: u32) -> Result<u8, Refused>;
    fn send(&mut self, handle: u32, bytes: &[u8]) -> Result<(), Refused>;
    /// Whether bytes sent on `handle` still wait for their answer, after
    /// asking once more for it. Through a proxy a send is taken at once and
    /// its answer may come ticks later, and nothing that depends on that
    /// answer may run before it. Refused when it can never come.
    fn sending(&mut self, handle: u32) -> Result<bool, Refused>;
    fn close(&mut self, handle: u32);
    /// Whether a connection that leaves `way` can be opened now: a proxy
    /// gives one program a few streams (`net::mixnet::streams`), and the
    /// browser asks for one only when it holds fewer. A direct connection
    /// always may.
    fn room(&self, _way: Way) -> bool {
        true
    }
    /// Why a proxy can be asked nothing more on `handle`, when that is not
    /// the far end finishing (`proxy_fault`). A read there says Closed all
    /// the same; this says whose close it was.
    fn broke(&self, _handle: u32) -> Option<&'static str> {
        None
    }
    /// One line for the debug console.
    fn trace(&mut self, line: &[u8]);
}
