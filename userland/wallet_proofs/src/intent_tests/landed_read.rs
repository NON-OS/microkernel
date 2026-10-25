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

/* Whether a sent publisher call landed: the READ asked (a view of
 * settle_proto's table, its words in their forms) and each kind's answer,
 * read strictly. */

use nonos_marketplace_abi::bond_id;
use nonos_nox_config::Contract;
use nonos_settle_proto::{view, view_calldata, KINDS};

use super::fixture::{at, n, K};
use super::kinds::every;
use crate::intent::{landed_read, listing_of, Publisher};

const FROM: [u8; 20] = [9; 20];

fn addr(a: [u8; 20]) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[12..].copy_from_slice(&a);
    w
}

#[test]
fn each_kind_reads_its_own_view() {
    let ids = [2, 30, 32, 30, 30, 40, 40, 50, 35];
    for (p, id) in every().iter().zip(ids) {
        let (got, args) = landed_read(&K, p, &FROM, &at).unwrap();
        let v = view(got).unwrap();
        assert_eq!(got, id, "{p:?}");
        assert!(view_calldata(v, &args).is_some(), "the words fit {}", v.signature);
        let kind = KINDS.iter().find(|k| k.id == p.kind()).unwrap();
        let on = if p.kind() == 1 { Contract::NoxToken } else { kind.contract };
        assert_eq!(v.contract, on, "{p:?}");
    }
    let e = every();
    let spender = addr(at(Contract::PublisherBond).unwrap());
    assert_eq!(landed_read(&K, &e[0], &FROM, &at).unwrap().1, vec![addr(FROM), spender]);
    assert_eq!(landed_read(&K, &e[0], &FROM, &|_| None), None, "no spender deployed");
    assert_eq!(landed_read(&K, &e[1], &FROM, &at).unwrap().1, vec![bond_id(&K, &FROM, &[1; 32])]);
    let listing = listing_of(&K, &[1; 32], &[4; 32]);
    assert_eq!(landed_read(&K, &e[5], &FROM, &at).unwrap().1, vec![listing, [5; 32]]);
    assert_eq!(landed_read(&K, &e[7], &FROM, &at).unwrap().1, vec![[8; 32], n(1), [7; 32]]);
    let credit = Publisher::WithdrawBondCredit { to: FROM };
    assert_eq!(landed_read(&K, &credit, &FROM, &at).unwrap(), (35, vec![addr(FROM)]));
}
