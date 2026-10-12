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

//! The delete after a renewal: refused, the new record is taken back and
//! the old one is still what a restart reads; already gone, the renewal
//! stands.

use crate::faulty::Faulty;
use crate::fixture::public;
use nonos_bt_bond::{BondError, BondRecord, Keys, KeysError, Store, KIND_BOND};
use nonos_bt_wire::addr::{AddrType, BdAddr};

fn kept_ltk(s: &Store<Faulty>) -> Option<[u8; 16]> {
    s.find(AddrType::Public, &BdAddr(public().identity)).map(|r| r.ltk)
}

fn fresh() -> BondRecord {
    BondRecord { ltk: *b"a-fresh-ltk-here", ..public() }
}

#[test]
fn a_refused_delete_leaves_only_the_old_record() {
    let mut s = Store::open(Faulty::default()).unwrap();
    s.keep(public()).unwrap();
    s.keys().delete_next = Some(KeysError::Denied);
    assert_eq!(s.keep(fresh()), Err(BondError::Keys(KeysError::Denied)));
    assert_eq!(s.keys().inner.list(KIND_BOND).unwrap().len(), 1);
    let s = Store::open(s.into_keys()).unwrap();
    assert_eq!(kept_ltk(&s), Some(public().ltk));
}

#[test]
fn an_old_record_already_gone_does_not_stop_the_renewal() {
    let mut s = Store::open(Faulty::default()).unwrap();
    s.keep(public()).unwrap();
    s.keys().delete_next = Some(KeysError::NotFound);
    assert_eq!(s.keep(fresh()), Ok(()));
    assert_eq!(kept_ltk(&s), Some(fresh().ltk));
    // The store forgets the old id, so forgetting the bond deletes the new.
    assert_eq!(s.forget(0, &public().identity), Ok(true));
    let ids = s.keys().inner.list(KIND_BOND).unwrap();
    assert_eq!(ids.len(), 1);
    let left = s.keys().inner.retrieve(ids[0]).unwrap();
    assert_eq!(BondRecord::decode(&left).unwrap().ltk, public().ltk);
}
