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

//! Op 19's two roles against every other caller: app.store alone posts
//! names and takes the orderId; the wallet's three names alone take names
//! and post the orderId.

use nonos_nox_meter::caller::Role;

fn registry(name: &[u8]) -> Option<u32> {
    match name {
        b"app.nonos_wallet" => Some(10),
        b"app.nonos_wallet.1" => Some(11),
        b"app.nonos_wallet.2" => Some(12),
        b"nox_settle" => Some(20),
        b"installer" => Some(30),
        b"app.store" => Some(31),
        b"app.other" => Some(40),
        b"app.linux.install" => Some(50),
        _ => None,
    }
}

/// The wallets, settle, the installer, the install role, a registered
/// name no role lists, and a pid that owns no name.
const NOT_THE_STORE: [u32; 8] = [10, 11, 12, 20, 30, 50, 40, 99];
const NOT_THE_WALLET: [u32; 6] = [31, 20, 30, 50, 40, 99];

#[test]
fn the_store_role_admits_app_store_and_nobody_else() {
    assert!(Role::Store.admits(31, registry));
    for pid in NOT_THE_STORE {
        assert!(!Role::Store.admits(pid, registry), "Store admitted {pid}");
    }
    let no_store = |n: &[u8]| if n == b"app.store" { None } else { registry(n) };
    assert!(!Role::Store.admits(31, no_store), "app.store unregistered");
    assert!(!Role::Store.admits(0, registry) && !Role::Store.admits(0, |_| Some(0)));
}

#[test]
fn the_wallet_role_admits_the_three_wallet_names_and_nobody_else() {
    for pid in [10, 11, 12] {
        assert!(Role::Wallet.admits(pid, registry), "Wallet refused {pid}");
    }
    for pid in NOT_THE_WALLET {
        assert!(!Role::Wallet.admits(pid, registry), "Wallet admitted {pid}");
    }
    assert!(!Role::Wallet.admits(0, registry) && !Role::Wallet.admits(0, |_| Some(0)));
}
