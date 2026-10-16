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

// The rule the wallet's requests leave by, as nonos_route_link ships it,
// re-exported at the root where its own `crate::pick` paths look for it.
mod route_link;
pub use route_link::{describe, pick};

#[allow(dead_code)]
#[path = "../../capsule_wallet_nonos/src/wallet/event/hex_digit.rs"]
pub mod hex_digit;
/// The real NOX read helpers. `apr_bps` reads `super::constants`, so `constants`
/// is a sibling module under `nox` exactly as in the wallet.
#[allow(dead_code)]
pub mod nox;

/// The keyring's `server` modules the words check names, under the
/// `crate::server` path it is written against.
pub mod server;
/// The keyring's key store, which holds the wallet's opened account key. At
/// the crate root, where its own `crate::store` paths look for it.
pub mod store;
pub mod wallet;

// The other mounted files, grouped by meaning and named at the crate root.
mod mounts_custody;
pub use mounts_custody::*;
mod mounts_market;
pub use mounts_market::*;
mod mounts_read;
pub use mounts_read::*;

#[allow(dead_code)]
pub fn source_root() -> &'static str {
    WALLET
}

#[cfg(kani)]
mod kani;
#[cfg(test)]
mod tests_custody;
#[cfg(test)]
mod tests_market;
#[cfg(test)]
mod tests_net;
#[cfg(test)]
mod tests_send;
