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

//! What settle keeps between calls: its RPC hosts, which config contracts
//! passed the start checks, and the one verified release and the one sealed
//! escrow answer FETCH pages from.

use alloc::vec::Vec;

use crate::chain::Live;
use crate::listing::Cursors;
use crate::net::Rpc;

pub struct Held {
    pub release_hash: [u8; 32],
    pub bytes: Vec<u8>,
}

/// The one escrow answer held, sealed, for the pid that asked (v1.4.24).
pub struct HeldEscrow {
    pub buyer_pk: [u8; 32],
    pub pid: u32,
    pub bytes: Vec<u8>,
    /// When it was fetched or last paged, in ms; ESCROW_IDLE_MS after, it goes.
    pub at_ms: u64,
}

pub struct Settle {
    pub rpc: Rpc,
    pub live: Live,
    pub held: Option<Held>,
    pub escrow: Option<HeldEscrow>,
    /// LISTING's per-chain cursors over the registry's logs.
    pub cursors: Cursors,
    /// DELIVERED's per-chain reading of the escrow's Delivered logs.
    pub delivered: alloc::vec::Vec<nonos_settle_proto::DeliveredKeys>,
}

impl Settle {
    /// Settle as it starts: every contract is checked once now. One a check
    /// could not reach is checked again when it is first used.
    pub fn start() -> Settle {
        let (rpc, live, cursors) = (Rpc::new(), Live::new(), Cursors::default());
        let delivered = alloc::vec::Vec::new();
        let mut s = Settle { rpc, live, held: None, escrow: None, cursors, delivered };
        s.live.check_all(&mut s.rpc);
        s
    }
}
