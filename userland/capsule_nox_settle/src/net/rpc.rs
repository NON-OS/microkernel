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

//! The RPC host settle asks on each chain: one at a time, the next after a
//! failure. A host swapped in must first say eth_chainId is the chain's, so
//! a host on another chain is never read as this one.

use alloc::vec::Vec;

use nonos_eth_rpc::{parse_u64, request_chain_id};
use nonos_settle_proto::{EINVAL, EIO, ENETDOWN, EPROTO};

use super::hosts::hosts;
use super::https::post;

/// The most an RPC answer may hold.
pub(super) const ANSWER_MAX: usize = 4 * 1024 * 1024;

pub struct Rpc {
    pub(super) at: [usize; 2],
    pub(super) checked: [bool; 2],
}

pub(super) fn on_chain(host: &str, chain_id: u64) -> Result<(), i32> {
    let said = post(host, &request_chain_id(1), 512)?;
    (parse_u64(&said) == Some(chain_id)).then_some(()).ok_or(EPROTO)
}

impl Rpc {
    pub fn new() -> Rpc {
        Rpc { at: [0; 2], checked: [false; 2] }
    }

    /// `body`'s answer from the chain's current host, trying each in turn.
    pub fn call(&mut self, chain_id: u64, body: &[u8]) -> Result<Vec<u8>, i32> {
        let (slot, list) = hosts(chain_id).ok_or(EINVAL)?;
        let mut last = EIO;
        for _ in 0..list.len() {
            let host = list[self.at[slot]];
            let asked = match self.checked[slot] {
                true => post(host, body, ANSWER_MAX),
                false => on_chain(host, chain_id).and_then(|_| post(host, body, ANSWER_MAX)),
            };
            match asked {
                Ok(answer) => {
                    self.checked[slot] = true;
                    return Ok(answer);
                }
                Err(ENETDOWN) => return Err(ENETDOWN),
                Err(e) => last = e,
            }
            self.at[slot] = (self.at[slot] + 1) % list.len();
            self.checked[slot] = false;
        }
        Err(last)
    }
}
