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

use nonos_nox_config::{Contract, MAINNET, SEPOLIA};
use nonos_settle_proto::kind_of;

use crate::intent::{calldata, encode, to, Publisher};

const AT: [u8; 20] = [0xAA; 20];

pub(crate) fn every() -> Vec<Publisher> {
    use Publisher::*;
    vec![
        Approve { spender: Contract::PublisherBond, amount: 5 },
        Bond { publisher_key: [1; 32], amount: 7 },
        SetDeliveryAgent { bond_id: [2; 32], agent: [3; 20] },
        RequestUnbond { bond_id: [2; 32] },
        WithdrawBond { bond_id: [2; 32] },
        Publish {
            publisher_key: [1; 32],
            app_id_hash: [4; 32],
            release_id: [5; 32],
            release_hash: [6; 32],
            bond_id: [2; 32],
            uri: b"https://example.org/r/qwen-small@1".to_vec(),
        },
        WithdrawRelease { listing_id: [7; 32], release_id: [5; 32] },
        Anchor { anchor: [8; 32], kind: 1, listing_id: [7; 32] },
        WithdrawBondCredit { to: [3; 20] },
    ]
}

#[test]
fn settle_takes_every_publisher_call_as_its_kind() {
    for p in every() {
        let cd = encode(&p, |_| Some(AT)).unwrap();
        let k = kind_of(p.kind(), &cd).unwrap_or_else(|| panic!("{p:?}"));
        assert_eq!(k.id, p.kind());
    }
}

#[test]
fn nothing_is_built_for_a_contract_not_deployed() {
    let none = |_| None;
    assert_eq!(encode(&every()[0], none), None, "approve names its spender");
    for chain in [MAINNET, SEPOLIA] {
        for p in every().iter().filter(|p| p.kind() != 1) {
            assert_eq!(to(p, chain), None, "{p:?}");
        }
        assert_eq!(calldata(&every()[0], chain), None);
    }
    assert_eq!(
        to(&every()[0], 11155111),
        Some(nonos_nox_config::address(11155111, Contract::NoxToken).unwrap())
    );
    assert_eq!(to(&every()[1], 5), None, "an unknown chain");
}
