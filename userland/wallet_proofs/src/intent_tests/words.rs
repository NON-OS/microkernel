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

use nonos_nox_config::Contract;
use sha3::{Digest, Keccak256};

use crate::intent::{encode, Publisher};

fn sel(sig: &str) -> Vec<u8> {
    Keccak256::digest(sig.as_bytes())[..4].to_vec()
}

fn n(v: u128) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[16..].copy_from_slice(&v.to_be_bytes());
    w
}

fn a(x: [u8; 20]) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[12..].copy_from_slice(&x);
    w
}

fn built(p: Publisher) -> Vec<u8> {
    let at = |c| (c == Contract::PublisherBond).then_some([0xBB; 20]);
    encode(&p, at).unwrap()
}

#[test]
fn static_calls_word_for_word() {
    use Publisher::*;
    let approve = Approve { spender: Contract::PublisherBond, amount: 10u128.pow(21) };
    let want =
        [sel("approve(address,uint256)"), a([0xBB; 20]).to_vec(), n(10u128.pow(21)).to_vec()];
    assert_eq!(built(approve), want.concat());
    let bond = Bond { publisher_key: [1; 32], amount: 9 };
    assert_eq!(built(bond), [sel("bond(bytes32,uint256)"), vec![1; 32], n(9).to_vec()].concat());
    let agent = SetDeliveryAgent { bond_id: [2; 32], agent: [3; 20] };
    let want = [sel("setDeliveryAgent(bytes32,address)"), vec![2; 32], a([3; 20]).to_vec()];
    assert_eq!(built(agent), want.concat());
    let anchor = Anchor { anchor: [8; 32], kind: 1, listing_id: [7; 32] };
    let want = [sel("anchor(bytes32,uint8,bytes32)"), vec![8; 32], n(1).to_vec(), vec![7; 32]];
    assert_eq!(built(anchor), want.concat());
    let credit = WithdrawBondCredit { to: [3; 20] };
    assert_eq!(built(credit), [sel("withdrawCredit(address)"), a([3; 20]).to_vec()].concat());
}
