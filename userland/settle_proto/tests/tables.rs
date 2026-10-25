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
use nonos_settle_proto::*;
use sha3::{Digest, Keccak256};

fn selector(signature: &str) -> u32 {
    let h = Keccak256::digest(signature.as_bytes());
    u32::from_be_bytes([h[0], h[1], h[2], h[3]])
}

fn params(signature: &str) -> usize {
    let inner = &signature[signature.find('(').unwrap() + 1..signature.len() - 1];
    inner.split(',').filter(|p| !p.is_empty()).count()
}

#[test]
fn every_view_selector_is_its_signatures_and_ids_are_unique() {
    for (i, v) in VIEWS.iter().enumerate() {
        assert_eq!(v.selector, selector(v.signature).to_be_bytes(), "{}", v.signature);
        assert_eq!(v.args.len(), params(v.signature), "{}", v.signature);
        assert!(VIEWS[..i].iter().all(|w| w.id != v.id), "view {} twice", v.id);
        assert_eq!(view(v.id), Some(v));
    }
    assert_eq!(view(60).unwrap().contract, Contract::ShieldPool);
    assert_eq!(view(14).unwrap().signature, "openNonce(address)");
    assert_eq!(view(61).unwrap().contract, Contract::ShieldPool);
    assert_eq!(view(75).unwrap().contract, Contract::AmountPolicy);
    for id in [0, 5, 19, 28, 38, 42, 51, 52, 62, 69, 76, 0xffff] {
        assert_eq!(view(id), None, "{id}");
    }
}

#[test]
fn every_kind_selector_is_its_signatures_and_kinds_run_1_to_20() {
    for (i, k) in KINDS.iter().enumerate() {
        assert_eq!(k.id as usize, i + 1);
        for (sig, sel) in k.calls {
            let full = sig.replace("(T", &format!("({ORDER_TERMS_TUPLE}"));
            let full = full.replace("(V[]", &format!("({VOUCHER_TUPLE}[]"));
            assert_eq!(*sel, selector(&full), "{sig}");
        }
    }
    assert_eq!(KINDS[0].contract, Contract::NoxToken);
    assert_eq!(KINDS[12].calls[0].1, 0x631969a5, "the meter's open");
    assert_eq!(KINDS[19].contract, Contract::ShieldPool, "ShieldClaim");
    let credit: Vec<_> = KINDS[16..19].iter().map(|k| (k.calls[0].1, k.contract)).collect();
    let owners = [Contract::Settlement, Contract::Escrow, Contract::PublisherBond];
    assert_eq!(credit, owners.map(|c| (0x6c8cc9d9, c)), "one call on three contracts");
}

#[test]
fn the_voucher_tuple_is_the_configs_voucher_type() {
    let t = nonos_nox_config::VOUCHER.type_string;
    let fields = &t[t.find('(').unwrap() + 1..t.len() - 1];
    let types: Vec<&str> = fields.split(',').map(|f| f.split(' ').next().unwrap()).collect();
    assert_eq!(format!("({})", types.join(",")), VOUCHER_TUPLE);
    assert_eq!(ORDER_TERMS_TUPLE.matches(',').count(), 13, "14 OrderTerms fields");
}
