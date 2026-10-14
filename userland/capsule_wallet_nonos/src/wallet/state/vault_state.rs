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
//! The keyring this window talks to, and the vault it keeps the wallet in:
//! whether it is saved, was tried, has gone quiet, or sits unopened.

#[derive(Default)]
pub struct VaultState {
    pub keyring_port: u32,
    pub owner_pid: u32,
    pub wallet_id: u32,
    /// Whether this wallet is on the disk sealed to this machine. False on a
    /// machine with no TPM, and the status line says so once.
    pub saved: bool,
    /// One attempt per window. A vault that will not open must not be retried
    /// on every hydrate, which would put a TPM derivation on a timer.
    pub restore_tried: bool,
    /// The uptime the vault's store first went unanswered, while it has
    /// not answered since: past the patience a new wallet is asked twice
    /// rather than refused (`event::replace_rule`).
    pub silent_since: Option<i64>,
    /// A vault is on the disk that this boot could not open or name: a new
    /// wallet over it is asked twice (`event::may_replace`).
    pub present: bool,
    /// What the last keep came to, when it did not keep the wallet across
    /// reboots: drawn on the phrase screen and on home, where the person
    /// decides whether to write the words down (`event::keep_plan`).
    pub kept_note: Option<&'static str>,
}
