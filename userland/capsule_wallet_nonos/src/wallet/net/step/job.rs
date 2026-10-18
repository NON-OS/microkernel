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

//! The network work a refresh does, held between ticks: the diagnostic,
//! the account snapshot, or a reading of the payments still on their way. Each tick steps
//! it once, and only a finished job's result reaches the state. A snapshot
//! remembers the account and network it was asked for, so one that lands
//! after a switch is dropped rather than shown against the wrong account.

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::wallet::net::WalletRoute;

use super::super::read_snapshot::{parse_snapshot, snapshot_request};
use super::exchange::{Ended, Exchange};
use super::job_follow::followed;
use super::probe::Probe;

pub use super::job_finished::{Finished, Followed};

pub enum Job {
    Probe(Probe),
    Snapshot { exchange: Exchange, address: [u8; 20], chain: u64 },
    Follow { exchange: Exchange, hashes: Vec<[u8; 32]>, address: [u8; 20], chain: u64 },
}

impl Job {
    pub fn probe() -> Job {
        Job::Probe(Probe::begin())
    }

    pub fn snapshot(address: &[u8; 20]) -> Job {
        let body = snapshot_request(address);
        let exchange = Exchange::begin(WalletRoute::for_wallet(), body);
        let chain = crate::wallet::chain::current().id;
        Job::Snapshot { exchange, address: *address, chain }
    }

    /// One step; the result once the job is finished.
    pub fn step(&mut self) -> Option<Finished> {
        match self {
            Job::Probe(probe) => probe.step().map(Finished::Probe),
            Job::Snapshot { exchange, address, chain } => {
                let snap = match exchange.step()? {
                    Ended::Answer(resp) => Some(Box::new(parse_snapshot(&resp))),
                    Ended::Failed(_) | Ended::Unknown(_) => None,
                };
                let host = exchange.host;
                Some(Finished::Snapshot { snap, address: *address, chain: *chain, host })
            }
            Job::Follow { exchange, hashes, address, chain } => {
                let followed = match exchange.step()? {
                    Ended::Answer(resp) => followed(&resp, hashes),
                    Ended::Failed(_) | Ended::Unknown(_) => None,
                };
                Some(Finished::Follow { followed, address: *address, chain: *chain })
            }
        }
    }
}
