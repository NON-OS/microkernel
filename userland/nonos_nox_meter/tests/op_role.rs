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

//! The op-to-role table the meter's dispatch asks before it reads a body
//! (caller::op_role): every op, each PURCHASE sub-op, and the refusals.

use nonos_nox_meter::caller::{op_role, purchase_role, Role};
use nonos_payment_proto::{Names, OP_OPEN, OP_PURCHASE};

fn registry(name: &[u8]) -> Option<u32> {
    match name {
        b"app.nonos_wallet" => Some(10),
        b"app.store" => Some(31),
        _ => None,
    }
}

#[test]
fn each_op_has_the_role_plan_21_gives_it() {
    use Role::*;
    let wallet: &[u16] = &[5, 6, 8, 11, 12, 13, 14, 16, 17, 20];
    let roles = [(Anyone, &[1, 4][..]), (Payer, &[2, 9, 15]), (Settle, &[3]), (Wallet, wallet)];
    // 19 with the body [0] is the store's POST; 21 is its FUNDED.
    let roles =
        [&roles[..], &[(Deliverer, &[10][..]), (Install, &[18]), (Store, &[19, 21])]].concat();
    for op in 0..=u16::MAX {
        let want = roles.iter().find(|(_, ops)| ops.contains(&op)).map(|(r, _)| *r);
        assert_eq!(op_role(op, &[0]), want, "op {op}");
    }
}

#[test]
fn each_purchase_sub_op_has_its_role_and_others_none() {
    for (sub, role) in [(0, Role::Store), (1, Role::Wallet), (2, Role::Wallet), (3, Role::Store)] {
        assert_eq!(purchase_role(&[sub]), Some(role), "19/{sub}");
        assert_eq!(op_role(OP_PURCHASE, &[sub, 9, 9]), Some(role), "19/{sub} with a body");
    }
    for body in [&[][..], &[4], &[0xff, 0]] {
        assert_eq!(op_role(OP_PURCHASE, body), None, "{body:?}");
    }
    let post = Names { chain_id: 1, listing: b"linux.a".to_vec(), release: Vec::new() };
    let role = op_role(OP_PURCHASE, &post.post().unwrap()).unwrap();
    assert!(role.admits(31, registry) && !role.admits(10, registry), "the store posts");
    let take = op_role(OP_PURCHASE, &[1]).unwrap();
    assert!(take.admits(10, registry) && !take.admits(31, registry), "the wallet takes");
}

#[test]
fn open_is_the_wallets_alone() {
    let role = op_role(OP_OPEN, &[]).unwrap();
    assert_eq!(role, Role::Wallet);
    assert!(role.admits(10, registry));
    assert!(!role.admits(31, registry), "app.store opens no channel");
}
