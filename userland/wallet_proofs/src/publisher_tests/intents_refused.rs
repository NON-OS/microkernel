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

//! Every call from the entry is first held to binding_plan's checks: an
//! entry not sold, paying another account, or naming another bond builds
//! nothing, and a release index past the entry's releases neither.

use nonos_marketplace_abi::{MarketplaceEntry, SettleMode};

use super::fixture::{entry, K, ME};
use super::intents::word;
use crate::intent::{agent_of, anchors_of, bond_of, publish_of, unbond_of, withdraw_bond_of};
use crate::intent::{withdraw_of, Why};
use crate::publisher::Refused;

const URI: &str = "https://example.org/r/qwen-small@1";

fn refusals(e: &MarketplaceEntry, me: &[u8; 20]) -> Vec<Option<Why>> {
    let unlocked = [word(0), word(0), word(0), word(1), word(0)];
    vec![
        bond_of(&K, e, me, 1_000, 1, &word(0)).err(),
        agent_of(&K, e, me, &format!("0x{}", "ab".repeat(20))).err(),
        unbond_of(&K, e, me).err(),
        withdraw_bond_of(&K, e, me, &unlocked, 2).err(),
        publish_of(&K, e, me, 0, URI).err(),
        withdraw_of(&K, e, me, 0).err(),
        anchors_of(&K, e, me, 0).err(),
    ]
}

#[test]
fn nothing_is_built_off_the_binding_plans_checks() {
    let e = entry(SettleMode::Escrow);
    assert!(refusals(&e, &ME).iter().all(Option::is_none), "{:?}", refusals(&e, &ME));
    let all = |r: Refused| vec![Some(Why::Entry(r)); 7];
    assert_eq!(refusals(&e, &[0x12; 20]), all(Refused::NotPayee));
    let mut other = entry(SettleMode::Escrow);
    other.commerce.as_mut().unwrap().bond_id[31] ^= 1;
    assert_eq!(refusals(&other, &ME), all(Refused::NotOurBond));
    let mut unsold = entry(SettleMode::Escrow);
    unsold.commerce = None;
    assert_eq!(refusals(&unsold, &ME), all(Refused::NotSold));
    let no = Err(Why::Entry(Refused::NoRelease));
    assert_eq!((publish_of(&K, &e, &ME, 2, URI), withdraw_of(&K, &e, &ME, 2)), (no.clone(), no));
    assert_eq!(anchors_of(&K, &e, &ME, 2), Err(Why::Entry(Refused::NoRelease)));
}
