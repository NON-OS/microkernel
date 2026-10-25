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

/* The review page is read back from the calldata signed: the same rows as
 * the Publisher asked for, every address in full EIP-55, ids whole, and
 * the function as KINDS writes it. */

use nonos_marketplace_abi::address_text;
use nonos_nox_config::Contract;
use nonos_settle_proto::{Quote, KINDS};

use super::decode::built;
use super::fixture::{at, nox, quote, K};
use super::kinds::every;
use crate::intent::{fee_rows, head_rows, hex, review_rows, rows, Publisher};

pub(crate) fn page(kind: u8, cd: &[u8]) -> Option<Vec<(&'static str, String)>> {
    rows(kind, cd, &quote(kind), &K, &at, nox, nox)
}

fn spelt(a: &[u8; 20]) -> String {
    String::from_utf8(address_text(&K, a).to_vec()).unwrap()
}

#[test]
fn the_page_is_the_calldatas() {
    for p in every() {
        let (q, kind) = (quote(p.kind()), p.kind());
        let head = head_rows(kind, &q.to, &K).unwrap();
        let want = [head.clone(), review_rows(&p, &K, &at, nox), fee_rows(&q, nox)].concat();
        assert_eq!(page(kind, &built(&p)), Some(want), "{p:?}");
        let sig = KINDS.iter().find(|k| k.id == kind).unwrap().calls[0].0;
        assert_eq!((head[1].0, head[1].1.as_str(), head[2].1.as_str()), ("function", sig, "0 ETH"));
        assert!(head[0].1.ends_with(&spelt(&q.to)), "{}", head[0].1);
        assert_eq!(rows(kind, &built(&p), &quote(kind % 8 + 1), &K, &at, nox, nox), None);
    }
    let other = Publisher::RequestUnbond { bond_id: [0xCD; 32] };
    let shown = page(4, &built(&other)).unwrap();
    assert!(shown.contains(&("bond", hex(&[0xCD; 32]))) && hex(&[0xCD; 32]).len() == 66);
    let q = Quote { to: [0x7E; 20], ..quote(4) };
    let signed_to = rows(4, &built(&other), &q, &K, &at, nox, nox).unwrap();
    assert!(signed_to[0].1.ends_with(&spelt(&[0x7E; 20])), "the `to` signed, not a default");
}

#[test]
fn addresses_in_full_eip55() {
    let agent = [0xAB, 0xCD, 0xEF, 0x01].repeat(5).try_into().unwrap();
    let p = Publisher::SetDeliveryAgent { bond_id: [2; 32], agent };
    let shown = page(3, &built(&p)).unwrap();
    let row = &shown.iter().find(|r| r.0 == "agent").unwrap().1;
    assert_eq!((row.as_str(), row.len()), (spelt(&agent).as_str(), 42));
    assert!(row.chars().any(|c| c.is_ascii_uppercase()), "{row}");
    let head = &shown[0].1;
    assert_eq!(*head, format!("NONOS Bond at {}", spelt(&at(Contract::PublisherBond).unwrap())));
}
