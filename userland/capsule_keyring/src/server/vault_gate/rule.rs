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

//! Who may seal and open the wallet's vault record.
//!
//! The record is named `keyring.wallet.account` and sealed under this
//! machine's root, so the blob opens for any capsule that presents it, and
//! the wallet keeps it at /data/wallet.vault, which any FileSystem holder can
//! read. Opening it under its own pid let a capsule sign or export the
//! wallet's key; sealing its own let it replace the wallet's account with one
//! it controls. Only the wallet, in any of its windows, may do either.
//!
//! Pure: the caller supplies who owns each name. Held in `wallet_proofs`.

/// The wallet's endpoints: its first window and the later ones
/// (`src/userspace/capsule_wallet_nonos/spawn.rs`).
pub const VAULT_HOLDERS: [&[u8]; 3] =
    [b"app.nonos_wallet", b"app.nonos_wallet.1", b"app.nonos_wallet.2"];

/// Whether `sender_pid` owns one of the wallet's endpoints, given
/// `owner_of`, the pid that owns a name.
pub fn may_use_vault(sender_pid: u32, owner_of: impl Fn(&[u8]) -> Option<u32>) -> bool {
    sender_pid != 0 && VAULT_HOLDERS.iter().any(|name| owner_of(name) == Some(sender_pid))
}
