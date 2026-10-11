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

use nonos_marketplace_abi::{bond_id, listing_id, SettleMode};

use super::fixture::{entry, K, KEY, ME};
use crate::publisher::{binding_plan, Refused};

#[test]
fn the_binding_shows_the_key_listing_and_payee_and_signs_its_words() {
    let p = binding_plan(&K, &entry(SettleMode::Escrow), &ME).unwrap();
    assert_eq!(p.rows[0].1, format!("0x{}", "22".repeat(32)));
    assert_eq!(p.rows[1].1, "linux.qwen-small");
    assert_eq!(p.rows[2].1, format!("0x{}", "11".repeat(20)));
    let listing = listing_id(&K, &KEY, "linux.qwen-small");
    assert_eq!(p.words, [KEY, listing, bond_id(&K, &ME, &KEY)]);
}

#[test]
fn a_binding_is_refused_for_another_payee_or_bond() {
    let r = |e, who: &[u8; 20]| binding_plan(&K, &e, who).err();
    assert_eq!(r(entry(SettleMode::Escrow), &[0x12; 20]), Some(Refused::NotPayee));
    let mut other = entry(SettleMode::Escrow);
    other.commerce.as_mut().unwrap().bond_id[0] ^= 1;
    assert_eq!(r(other, &ME), Some(Refused::NotOurBond));
    let mut unsold = entry(SettleMode::Escrow);
    unsold.commerce = None;
    assert_eq!(r(unsold, &ME), Some(Refused::NotSold));
}
