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

//! FUNDED (op 21) is app.store's alone; ACCOUNT (op 5) stays the wallet's,
//! whatever either body says.

use nonos_nox_meter::caller::{op_role, Role};
use nonos_payment_proto::{FundedAsk, OP_ACCOUNT, OP_FUNDED};

fn registry(name: &[u8]) -> Option<u32> {
    match name {
        b"app.nonos_wallet" => Some(10),
        b"app.nonos_wallet.1" => Some(11),
        b"installer" => Some(30),
        b"app.store" => Some(31),
        b"app.other" => Some(40),
        _ => None,
    }
}

#[test]
fn funded_admits_app_store_and_nobody_else() {
    let ask = FundedAsk { chain_id: 1, listing: b"linux.a".to_vec() }.encode().unwrap();
    for body in [&ask[..], &[], &[0], &[1]] {
        let role = op_role(OP_FUNDED, body);
        assert_eq!(role, Some(Role::Store), "{body:?}");
        let role = role.unwrap();
        assert!(role.admits(31, registry), "{body:?}");
        for pid in [0, 10, 11, 30, 40, 99] {
            assert!(!role.admits(pid, registry), "{pid} asked FUNDED");
        }
    }
}

#[test]
fn account_stays_the_wallets_and_refuses_app_store() {
    let ask = FundedAsk { chain_id: 1, listing: b"linux.a".to_vec() }.encode().unwrap();
    for body in [&[0][..], &[1], &ask] {
        let role = op_role(OP_ACCOUNT, body);
        assert_eq!(role, Some(Role::Wallet), "{body:?}");
        let role = role.unwrap();
        assert!(role.admits(10, registry) && role.admits(11, registry));
        assert!(!role.admits(31, registry), "app.store reads no ACCOUNT rows");
    }
}
