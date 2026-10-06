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

//! The keyring seals and opens the wallet's vault record for the wallet
//! alone. On the old keyring any capsule that read /data/wallet.vault could
//! open it under its own pid and then sign with or export the wallet's key:
//! `another_capsule_cannot_open_the_wallets_record` fails there.

use crate::vault_gate::{may_use_vault, VAULT_HOLDERS};

const WALLET: u32 = 40;
const WALLET_2: u32 = 52;

/// The registry as the keyring sees it: the wallet's first and third
/// windows, and other capsules under their own names.
fn registry(name: &[u8]) -> Option<u32> {
    match name {
        b"app.nonos_wallet" => Some(WALLET),
        b"app.nonos_wallet.2" => Some(WALLET_2),
        b"app.browser" => Some(61),
        b"app.terminal" => Some(62),
        _ => None,
    }
}

#[test]
fn the_wallet_in_any_window_may() {
    assert!(may_use_vault(WALLET, registry));
    assert!(may_use_vault(WALLET_2, registry));
}

#[test]
fn another_capsule_cannot_open_the_wallets_record() {
    for pid in [61, 62, 7, 1, u32::MAX] {
        assert!(!may_use_vault(pid, registry), "pid {pid}");
    }
}

#[test]
fn the_kernel_pid_and_an_absent_wallet_admit_nothing() {
    assert!(!may_use_vault(0, |_| Some(0)), "pid 0 is never the wallet");
    assert!(!may_use_vault(WALLET, |_| None), "no wallet registered, no vault");
}

/// The names are the wallet's: the one its spawn registers and its windows.
#[test]
fn the_holders_are_the_wallets_endpoints() {
    let spawn = include_str!("../../../src/userspace/capsule_wallet_nonos/spawn.rs");
    assert!(spawn.contains("const SERVICE_NAME: &str = \"app.nonos_wallet\";"));
    for name in VAULT_HOLDERS {
        let quoted = format!("\"{}\"", core::str::from_utf8(name).unwrap());
        assert!(spawn.contains(&quoted), "{quoted} is not a wallet endpoint");
    }
    let windows = spawn.matches("InstanceEndpoint {").count();
    assert_eq!(VAULT_HOLDERS.len(), 1 + windows, "a wallet window the gate does not name");
}
