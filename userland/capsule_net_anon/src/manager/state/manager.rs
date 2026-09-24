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

//! Every piece of state the transport keeps.

extern crate alloc;

use alloc::vec::Vec;

use crate::circuit::Circuit;
use crate::directory::consensus::Entry;
use crate::directory::microdesc::Microdesc;
use crate::directory::verify::AuthorityCert;
use crate::link::Link;
use crate::path::{Relay, Weights};
use crate::stream::Stream;

use super::super::guard::Guard;
use super::bootstrap::Bootstrap;

pub struct Manager {
    pub tcp_port: u32,
    pub bootstrap: Bootstrap,
    /*
     * The earliest time the bootstrap may try again. A stack with no address of
     * its own refuses every connect instantly, so retrying on the next turn swept
     * all seven authorities per turn: one boot measured 147 refusals.
     */
    pub retry_after: u64,
    /// Certificates that anchored, by authority index.
    pub certs: Vec<(usize, AuthorityCert)>,
    /// Consensus entries, kept while their microdescriptors arrive.
    pub entries: Vec<Entry>,
    /// Microdescriptors that have arrived and matched a digest the consensus
    /// named, keyed by that digest.
    pub micro: Vec<([u8; 32], Microdesc)>,
    /// Which request of the batch list is next, so one goes out per turn.
    pub micro_cursor: usize,
    /// Relays with both directory halves present, so every one is usable.
    pub relays: Vec<Relay>,
    pub weights: Weights,
    /// When the consensus stops being fresh and a new one should be fetched.
    pub fresh_until: u64,
    /// When it stops being valid and its relays must not be used at all.
    pub valid_until: u64,
    /// One link to one guard, shared by every circuit, as a relay expects.
    pub link: Option<Link>,
    pub circuits: Vec<Circuit>,
    pub streams: Vec<Stream>,
    pub next_circuit: u32,
    pub next_stream: u16,
    /// Where the next batched fetch starts, so one authority is not asked for
    /// the whole directory every time.
    pub authority_cursor: usize,
    /// Where the next stream starts looking for a circuit. Without it every
    /// stream took the first open one and a whole session went out of one exit.
    pub circuit_cursor: usize,
    /// The guard this session uses, kept across link losses so a broken
    /// connection cannot walk the client onto somebody else's relay.
    pub guard: Option<Guard>,
}
