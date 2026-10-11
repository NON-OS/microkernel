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

//! The wallet's own files the proofs check, mounted by path and re-exported
//! at the crate root, where the tests name them.

#[path = "../../capsule_wallet_nonos/src/wallet/accounts/file.rs"]
pub mod accounts_file;
#[path = "../../capsule_wallet_nonos/src/wallet/event/address_text.rs"]
pub mod address_text;
#[path = "../../capsule_wallet_nonos/src/wallet/send/exact.rs"]
pub mod exact;
#[path = "../../capsule_wallet_nonos/src/wallet/send/fees.rs"]
pub mod fees;
#[path = "../../capsule_wallet_nonos/src/wallet/event/idle_lock.rs"]
pub mod idle_lock;
#[path = "../../capsule_wallet_nonos/src/wallet/event/keep_plan.rs"]
pub mod keep_plan;
#[path = "../../capsule_wallet_nonos/src/wallet/shield/kept_names.rs"]
pub mod kept_names;
#[path = "../../capsule_wallet_nonos/src/wallet/event/keyring_says.rs"]
pub mod keyring_says;
#[path = "../../capsule_wallet_nonos/src/wallet/net/last_read.rs"]
pub mod last_read;
#[path = "../../capsule_wallet_nonos/src/wallet/etna/layout.rs"]
pub mod layout;
#[path = "../../capsule_wallet_nonos/src/wallet/event/reading.rs"]
pub mod reading;
#[path = "../../capsule_wallet_nonos/src/wallet/tls13/records_whole.rs"]
pub mod records_whole;
#[path = "../../capsule_wallet_nonos/src/wallet/event/replace_rule.rs"]
pub mod replace_rule;
#[path = "../../capsule_wallet_nonos/src/wallet/send/sent.rs"]
pub mod sent;
#[path = "../../capsule_wallet_nonos/src/wallet/screen/shield/typed.rs"]
pub mod shield_typed;
#[path = "../../capsule_wallet_nonos/src/wallet/screen/status_words.rs"]
pub mod status_words;
