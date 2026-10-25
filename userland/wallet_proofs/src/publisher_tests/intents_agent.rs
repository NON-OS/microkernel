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

//! The delivery agent as typed (Q3): mixed case only as its EIP-55
//! spelling, one case taken and shown checksummed, the zero address and
//! anything not one address refused.

use nonos_marketplace_abi::{address_text, SettleMode};

use super::fixture::{entry, K, ME};
use crate::intent::{agent_of, review_rows, Publisher, Why};

#[test]
fn the_agent_is_typed_and_its_case_is_its_checksum() {
    let agent: [u8; 20] = [0xAB, 0xCD, 0xEF, 0x01].repeat(5).try_into().unwrap();
    let spelt = String::from_utf8(address_text(&K, &agent).to_vec()).unwrap();
    let e = entry(SettleMode::Escrow);
    let typed = |t: &str| agent_of(&K, &e, &ME, t);
    let bond_id = e.commerce.as_ref().unwrap().bond_id;
    let call = vec![Publisher::SetDeliveryAgent { bond_id, agent }];
    let lower = spelt.to_ascii_lowercase();
    let upper = format!("0x{}", spelt[2..].to_ascii_uppercase());
    for t in [spelt.as_str(), &lower, &upper, &format!("  {}  ", &spelt[2..])] {
        assert_eq!(typed(t), Ok(call.clone()), "{t}");
    }
    let rows = review_rows(&call[0], &K, &|_| None, |v| v.to_string());
    assert!(rows.contains(&("agent", spelt.clone())), "the review shows {spelt}");
}

#[test]
fn a_wrong_case_the_zero_address_or_no_address_is_refused() {
    let agent: [u8; 20] = [0xAB, 0xCD, 0xEF, 0x01].repeat(5).try_into().unwrap();
    let spelt = String::from_utf8(address_text(&K, &agent).to_vec()).unwrap();
    let e = entry(SettleMode::Escrow);
    let typed = |t: &str| agent_of(&K, &e, &ME, t);
    for flip in (2..42).filter(|i| spelt.as_bytes()[*i].is_ascii_alphabetic()) {
        let mut bad = spelt.clone().into_bytes();
        bad[flip] ^= 0x20;
        let bad = String::from_utf8(bad).unwrap();
        assert_eq!(typed(&bad), Err(Why::Checksum), "{bad}");
    }
    assert_eq!(typed(&format!("0x{}", "0".repeat(40))), Err(Why::ZeroAgent));
    for t in [&spelt[..41], "", "0x", &format!("{spelt}0"), "0xabcdef01 abcdef01"] {
        assert_eq!(typed(t), Err(Why::NotAnAddress), "{t}");
    }
}
