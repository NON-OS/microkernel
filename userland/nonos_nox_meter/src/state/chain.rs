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

use super::meter::MeterState;
use crate::channel::NO_SIG;

/// One channel as settle read it from the contract (CHANNEL_STATE).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ChainView {
    pub deposit: u128,
    pub settled: u128,
    pub expiry: u64,
    pub closed: bool,
}

impl MeterState {
    /// The state after the chain's word on channel `id`. A closed channel
    /// holds nothing past what it paid, so its deposit becomes its settled
    /// amount and it takes no further charge. The contract's expiry is the
    /// channel's; a voucher signed under another would be refused, so its
    /// signature is dropped if they ever differ.
    pub fn from_chain(&self, id: &[u8; 32], v: &ChainView) -> MeterState {
        let deposit = if v.closed { v.settled } else { v.deposit };
        let mut next = self.synced(id, deposit, v.settled);
        if let Some(c) = next.channels.iter_mut().find(|c| &c.id == id) {
            if c.expiry != v.expiry {
                c.expiry = v.expiry;
                c.sig = NO_SIG;
            }
        }
        next
    }
}
