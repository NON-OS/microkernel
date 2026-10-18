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

//! The record a pairing leaves: kept by the identity the peer sent, else by
//! the address it connected from if that is an identity, else not at all.

use nonos_bt_bond::{BondError, BondRecord};
use nonos_bt_smp::pairing::Bond;
use nonos_bt_wire::addr::{AddrType, BdAddr};

const LTK: [u8; 16] = *b"derived-ltk-here";
const STATIC: BdAddr = BdAddr([9, 8, 7, 6, 5, 0xc4]);

fn bond(identity: Option<(u8, [u8; 6])>) -> Bond {
    let irk = identity.map(|_| *b"irk-from-pairing");
    Bond { ltk: LTK, irk, identity, authenticated: true }
}

#[test]
fn the_distributed_identity_is_kept() {
    let rpa = BdAddr([1, 2, 3, 4, 5, 0x45]);
    let r = BondRecord::from_pairing(&bond(Some((1, STATIC.0))), AddrType::Random, &rpa).unwrap();
    assert_eq!((r.identity_type, r.identity, r.ltk), (1, STATIC.0, LTK));
    assert_eq!((r.irk, r.authenticated, r.key_size), (Some(*b"irk-from-pairing"), true, 16));
}

#[test]
fn without_one_a_public_or_static_address_is_the_identity() {
    let public = BdAddr([1, 2, 3, 4, 5, 6]);
    let r = BondRecord::from_pairing(&bond(None), AddrType::Public, &public).unwrap();
    assert_eq!((r.identity_type, r.identity, r.irk), (0, public.0, None));
    let r = BondRecord::from_pairing(&bond(None), AddrType::Random, &STATIC).unwrap();
    assert_eq!((r.identity_type, r.identity), (1, STATIC.0));
}

#[test]
fn a_private_address_with_no_identity_cannot_be_kept() {
    for top in [0x45, 0x05] {
        let private = BdAddr([1, 2, 3, 4, 5, top]);
        let r = BondRecord::from_pairing(&bond(None), AddrType::Random, &private);
        assert_eq!(r, Err(BondError::NoIdentity));
    }
}
