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

//! The caller rule against a registry built here.

use nonos_nox_meter::caller::Role;

fn registry(name: &[u8]) -> Option<u32> {
    match name {
        b"app.nonos_wallet.1" => Some(10),
        b"nox_settle" => Some(20),
        b"installer" => Some(30),
        b"app.store" => Some(31),
        b"app.other" => Some(40),
        _ => None,
    }
}

#[test]
fn each_role_admits_only_the_owner_of_one_of_its_names() {
    assert!(Role::Wallet.admits(10, registry));
    assert!(Role::Payer.admits(10, registry));
    assert!(Role::Payer.admits(30, registry));
    assert!(Role::Payer.admits(31, registry));
    assert!(Role::Settle.admits(20, registry));
    assert!(Role::Buyer.admits(10, registry) && Role::Buyer.admits(31, registry));
    assert!(!Role::Buyer.admits(30, registry) && !Role::Wallet.admits(31, registry));
    for (role, pid) in
        [(Role::Wallet, 20), (Role::Wallet, 30), (Role::Settle, 10), (Role::Payer, 40)]
    {
        assert!(!role.admits(pid, registry), "{role:?} admitted {pid}");
    }
}

#[test]
fn anyone_means_any_named_sender_and_pid_zero_is_nobody() {
    assert!(Role::Anyone.admits(40, registry));
    assert!(Role::Anyone.admits(99, |_| None));
    for role in [Role::Anyone, Role::Wallet, Role::Settle, Role::Payer, Role::Buyer] {
        assert!(!role.admits(0, |_| Some(0)));
    }
    // A name nobody registered admits nobody.
    assert!(!Role::Wallet.admits(10, |_| None));
}
