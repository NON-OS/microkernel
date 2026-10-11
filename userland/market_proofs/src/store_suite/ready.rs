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

//! The orderId READY hands back is the store's only for the names it posted
//! (v1.4.45): chain, listing and release, all three.

use nonos_payment_proto::{Names, Ready};

use crate::store::purchase_said::ours;

#[test]
fn a_ready_for_other_names_is_never_this_order() {
    let names = |l: &[u8]| Names { chain_id: 11_155_111, listing: l.to_vec(), release: vec![] };
    let r = Ready { names: names(b"linux.a"), order_id: [7; 32] };
    assert_eq!(ours(Some(r.clone()), Some(&names(b"linux.a"))), Some(r.clone()));
    assert_eq!(ours(Some(r.clone()), Some(&names(b"linux.b"))), None, "another listing");
    let other_chain = Names { chain_id: 1, ..names(b"linux.a") };
    assert_eq!(ours(Some(r.clone()), Some(&other_chain)), None, "another chain");
    assert_eq!(ours(Some(r), None), None, "nothing posted");
}
