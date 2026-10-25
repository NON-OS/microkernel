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

//! The custody files: the vault's gate, the keyring's answers, kept records and status words.

#[path = "../../capsule_keyring/src/server/vault_gate/rule.rs"]
pub mod vault_gate;
/// The keyring's rule for who may keep records by name: Bluetooth bonds (L33).
#[path = "../../capsule_keyring/src/server/bond_gate/rule.rs"]
pub mod bond_gate;
// Keep the doc string referenced so the path note is not dead.
#[path = "../../capsule_wallet_nonos/src/wallet/accounts/file.rs"]
pub mod accounts_file;
#[path = "../../capsule_wallet_nonos/src/wallet/event/address_text.rs"]
pub mod address_text;
#[path = "../../capsule_wallet_nonos/src/wallet/event/locking/idle_lock.rs"]
pub mod idle_lock;
#[path = "../../capsule_wallet_nonos/src/wallet/event/locking/keep_plan.rs"]
pub mod keep_plan;
#[path = "../../capsule_wallet_nonos/src/wallet/event/locking/kept_status.rs"]
pub mod kept_status;
#[path = "../../capsule_wallet_nonos/src/wallet/event/custody/keyring_says.rs"]
pub mod keyring_says;
#[path = "../../capsule_wallet_nonos/src/wallet/ipc/keyring_wait.rs"]
pub mod keyring_wait;
#[path = "../../capsule_wallet_nonos/src/wallet/event/custody/replace_rule.rs"]
pub mod replace_rule;
#[path = "../../capsule_wallet_nonos/src/wallet/screen/status_words.rs"]
pub mod status_words;
