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

//! The bond cache's ASYNC READs through settle; one not sent fails at once.

use nonos_libc::{mk_ipc_send, mk_service_lookup};
use nonos_settle_proto::{request_async, Read, OP_READ, SERVICE};

use super::cache::BondCache;
use super::refresh::Ask;

/// The loop wakes this often while a read is in flight, to drop a late one.
const TICK_MS: u64 = 1_000;

#[derive(Default)]
pub struct Bonds {
    pub cache: BondCache,
    pub(super) next_req: u32,
    /// The read in flight: its request id, and settle's pid when sent.
    pub(super) pending: Option<(u32, u32)>,
}

pub(super) fn settle() -> Option<(u32, u32)> {
    let (mut port, mut pid) = (0, 0);
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
    (rc == 0 && pid != 0).then_some((port, pid))
}

impl Bonds {
    pub fn wait_ms(&self) -> u64 {
        TICK_MS.saturating_mul(u64::from(self.cache.waiting()))
    }

    /// Start the queue's next refresh, if none runs (plan/28, v1.4.23).
    pub fn pump(&mut self, now_ms: u64) {
        let ask = self.cache.next(now_ms);
        self.send(ask, now_ms);
    }

    pub fn tick(&mut self, now_ms: u64) {
        let ask = self.cache.tick(now_ms);
        self.pending = self.pending.filter(|_| self.cache.waiting());
        self.send(ask, now_ms);
    }

    pub(super) fn send(&mut self, mut ask: Option<Ask>, now_ms: u64) {
        while let Some(a) = ask {
            self.next_req = self.next_req.wrapping_add(1);
            let body = Read { chain_id: a.chain, view: a.view, args: a.args }.encode();
            let tx = body.and_then(|b| request_async(OP_READ, self.next_req, &b));
            if let (Some((port, pid)), Some(tx)) = (settle(), tx) {
                if mk_ipc_send(port as u64, tx.as_ptr(), tx.len()) >= 0 {
                    self.pending = Some((self.next_req, pid));
                    return;
                }
            }
            ask = self.cache.on_answer(None, now_ms);
        }
    }
}
