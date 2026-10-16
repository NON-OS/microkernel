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

//! The other half of a two-host read: a host that is not the current one.

use alloc::vec::Vec;

use nonos_settle_proto::{EIO, ENETDOWN};

use super::https::post;
use super::rpc::{on_chain, Rpc, ANSWER_MAX};

impl Rpc {
    /// `body`'s answer from a second host, never the current one, its chain
    /// checked each time: the other half of a two-host read.
    pub fn call_second(&mut self, chain_id: u64, body: &[u8]) -> Result<Vec<u8>, i32> {
        let [_, rest @ ..] = self.slots.of(chain_id)?.hosts;
        let mut last = EIO;
        for host in rest {
            match on_chain(host, chain_id).and_then(|_| post(host, body, ANSWER_MAX)) {
                Ok(answer) => return Ok(answer),
                Err(ENETDOWN) => return Err(ENETDOWN),
                Err(e) => last = e,
            }
        }
        Err(last)
    }
}
