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

//! Proofs for the wallet's pure on-chain read helpers. The modules below are the
//! real files from `capsule_wallet_nonos`, included verbatim so the harnesses
//! check shipping code rather than a copy. Every property proved here has a
//! matching Lean theorem in `verification/lean/Nonos/Wallet*.lean`.

// The capsule's no_std sources reach the heap through `alloc`; declaring it
// lets them compile unchanged against the host's std.
extern crate alloc;

const WALLET: &str = "../capsule_wallet_nonos/src/wallet";

pub mod wallet;

// The rule the wallet's requests leave by, as nonos_route_link ships it. At
// the crate root, where its own `crate::pick` paths look for it.
#[allow(dead_code)]
#[path = "../../nonos_route_link/src/describe.rs"]
pub mod describe;
#[allow(dead_code)]
#[path = "../../nonos_route_link/src/pick.rs"]
pub mod pick;
/// The keyring's key store, which holds the wallet's opened account key. At
/// the crate root, where its own `crate::store` paths look for it.
pub mod store;
/// The keyring's rule for who may seal and open the wallet's vault record.
#[path = "../../capsule_keyring/src/server/vault_gate/rule.rs"]
pub mod vault_gate;

/// The keyring's `server` modules the words check names, under the
/// `crate::server` path it is written against.
pub mod server;

#[cfg(test)]
mod words_own_tests;

// The real NOX read helpers. `apr_bps` reads `super::constants`, so `constants`
// is a sibling module under `nox` exactly as in the wallet.
#[allow(dead_code)]
pub mod nox;

#[allow(dead_code)]
#[path = "../../capsule_wallet_nonos/src/wallet/event/hex_digit.rs"]
pub mod hex_digit;

// Keep the doc string referenced so the path note is not dead.
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
#[cfg(test)]
mod vault_read;

#[allow(dead_code)]
pub fn source_root() -> &'static str {
    WALLET
}

#[cfg(test)]
mod accounts_file_tests;
#[cfg(test)]
mod address_text_tests;
#[cfg(test)]
mod amount_tests;
#[cfg(test)]
mod fees_tests;
#[cfg(test)]
mod fetch_tests;
#[cfg(test)]
mod format_typed_tests;
#[cfg(test)]
mod keep_tests;
#[cfg(test)]
mod kept_names_tests;
#[cfg(test)]
mod keyring_owner_tests;
#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod mul_div_tests;
#[cfg(test)]
mod network_tests;
#[cfg(test)]
mod replace_tests;
#[cfg(test)]
mod scale_tests;
#[cfg(test)]
mod sent_tests;
#[cfg(test)]
mod shield_typed_tests;
#[cfg(test)]
mod stakeable_tests;
#[cfg(test)]
mod swap_curve_tests;
#[cfg(test)]
mod swap_limits_tests;
#[cfg(test)]
mod vault_gate_tests;

#[cfg(kani)]
mod kani_proofs;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod probe_model;

#[cfg(test)]
mod route_tests;

#[cfg(test)]
mod gather_tests;

#[cfg(test)]
mod send_tests;
