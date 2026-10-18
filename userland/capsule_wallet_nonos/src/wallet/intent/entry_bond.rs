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
 * Unbonding and the Bond's two withdrawals from the entry: the bond only
 * once bondOf (READ 30) says it is unlocked, the credit (READ 35) to this
 * account and only when there is some. INTENT's refusal is the backstop.
 */

use alloc::vec::Vec;

use nonos_marketplace_abi::{Keccak, MarketplaceEntry};

use super::abi::word_u64;
use super::call::Publisher;
use super::entry::Calls;
use super::why::{ours, Why};

pub fn unbond_of(k: &impl Keccak, e: &MarketplaceEntry, me: &[u8; 20]) -> Calls {
    Ok(Vec::from([Publisher::RequestUnbond { bond_id: ours(k, e, me)?.bond_id }]))
}

/// Q6, for greying: bondOf's five words, its unlockAt set and reached.
pub fn may_withdraw(bond: &[[u8; 32]], now: u64) -> bool {
    let [_, _, _, unlock, _] = bond else { return false };
    word_u64(unlock).is_some_and(|u| u != 0 && now >= u)
}

pub fn withdraw_bond_of(
    k: &impl Keccak,
    e: &MarketplaceEntry,
    me: &[u8; 20],
    bond: &[[u8; 32]],
    now: u64,
) -> Calls {
    let bond_id = ours(k, e, me)?.bond_id;
    let call = Vec::from([Publisher::WithdrawBond { bond_id }]);
    may_withdraw(bond, now).then_some(call).ok_or(Why::Locked)
}

/// Q7: to this account, refused (greyed) while the credit word is 0.
pub fn credit_of(me: &[u8; 20], credit: &[u8; 32]) -> Calls {
    let call = Vec::from([Publisher::WithdrawBondCredit { to: *me }]);
    (*credit != [0; 32]).then_some(call).ok_or(Why::NoCredit)
}
