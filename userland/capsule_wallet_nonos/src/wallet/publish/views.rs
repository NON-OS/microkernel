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

/*
 * The chain views the publisher screen's rows wait on, read by settle's
 * READ for an entry that is this account's: bondOf (30) greys Withdraw
 * the bond until it unlocks (Q6), credit (35) the Bond's credit at 0
 * (Q7); MIN_BOND (33) and NOX allowance (2) build the bond (Q1, Q2).
 * A row whose view has not answered stays greyed: none is guessed.
 */

use alloc::vec::Vec;

use nonos_marketplace_abi::{Keccak, MarketplaceEntry};
use nonos_nox_config::{address, Contract};
use nonos_settle_proto::Read;

use crate::wallet::intent::{landed_read, unbond_of, Publisher};

#[derive(Default)]
pub struct Views {
    pub chain_id: u64,
    pub bond: Option<Vec<[u8; 32]>>,
    pub credit: Option<[u8; 32]>,
    pub min_bond: Option<[u8; 32]>,
    pub allowance: Option<[u8; 32]>,
    pub next_ms: u64,
}

impl Views {
    /// The first view not yet read, as a READ; None once all are in, or
    /// for an entry that is not this account's.
    pub fn next(&self, k: &impl Keccak, e: &MarketplaceEntry, me: &[u8; 20]) -> Option<Read> {
        let at = |c| address(self.chain_id, c).ok();
        let read = |p: &Publisher| landed_read(k, p, me, &at);
        let (view, args) = match () {
            _ if self.bond.is_none() => read(unbond_of(k, e, me).ok()?.first()?)?,
            _ if self.credit.is_none() => read(&Publisher::WithdrawBondCredit { to: *me })?,
            _ if self.min_bond.is_none() => (33, Vec::new()),
            // allowance(me, Bond): the read names the spender, not an amount.
            _ if self.allowance.is_none() => {
                read(&Publisher::Approve { spender: Contract::PublisherBond, amount: 0 })?
            }
            _ => return None,
        };
        Some(Read { chain_id: self.chain_id, view, args })
    }

    /// The words `view` answered; a one-word view takes exactly one.
    /// Whether they were taken.
    pub fn took(&mut self, view: u16, words: Vec<[u8; 32]>) -> bool {
        let one = <[[u8; 32]; 1]>::try_from(words.as_slice()).ok().map(|[w]| w);
        match view {
            30 => self.bond = Some(words),
            35 => self.credit = one,
            33 => self.min_bond = one,
            _ => self.allowance = one,
        }
        view == 30 || one.is_some()
    }
}
