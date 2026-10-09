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
use crate::path::{Relay, Taken, Weights};
use crate::stream::Stream;

use super::super::dir_job::DirWork;
use super::super::guard::Guard;
use super::super::onion::OnionJob;
use crate::onion::cache::DescCache;
use crate::onion::client_auth::ClientKey;
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
    /// Authorities whose held certificate the last consensus said is not
    /// the key it was signed with. The next sweep asks them again; the
    /// certificate held stays until a newer one anchors in its place.
    pub refetch: Vec<usize>,
    /// Consensus entries, kept while their microdescriptors arrive.
    pub entries: Vec<Entry>,
    /// Microdescriptors that have arrived and matched a digest the consensus
    /// named, keyed by that digest.
    pub micro: Vec<([u8; 32], Microdesc)>,
    /// The directory fetch in flight and where each sweep stands.
    pub dir: DirWork,
    /// Relays with both directory halves present, so every one is usable.
    pub relays: Vec<Relay>,
    pub weights: Weights,
    /// The consensus's `valid-after`: onion lookups take their time period
    /// from it, not from this machine's clock.
    pub valid_after: u64,
    /// When the consensus stops being fresh and a new one should be fetched.
    pub fresh_until: u64,
    /// When it stops being valid and its relays must not be used at all.
    pub valid_until: u64,
    /// Authorities whose signature on the consensus in use verified.
    pub consensus_signatures: u8,
    /// The consensus's shared random values, for the HSDir ring.
    pub srv_current: Option<[u8; 32]>,
    pub srv_previous: Option<[u8; 32]>,
    /// Onion service lookups in progress, one per stream being opened.
    pub onion: Vec<OnionJob>,
    /// Descriptors fetched and checked, held in memory only.
    pub desc_cache: DescCache,
    /// Client authorization keys, each with the pid that gave it. Memory
    /// only; the secrets are wiped when an entry goes.
    pub client_keys: Vec<(ClientKey, u32)>,
    /// Short names: the signed list, this boot's pins, and its fetch.
    pub names: crate::manager::names::NameState,
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
    /// Guards this boot gave up on, which a redraw avoids.
    pub given_up: Vec<Taken>,
    /// A consensus that was Ready is being replaced; the relays, weights and
    /// times in hand serve until the new set is whole (refresh_rule.rs).
    pub refreshing: bool,
    /// The new consensus's times, weights and signatures, held during a
    /// refresh until its relays are swapped in.
    pub next: Option<NextConsensus>,
}

/// What a refreshed consensus brings beyond its relays.
#[derive(Clone)]
pub struct NextConsensus {
    pub valid_after: u64,
    pub fresh_until: u64,
    pub valid_until: u64,
    pub weights: Weights,
    pub signatures: u8,
    pub srv_current: Option<[u8; 32]>,
    pub srv_previous: Option<[u8; 32]>,
}
