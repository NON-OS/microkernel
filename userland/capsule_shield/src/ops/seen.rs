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

//! What a state request answers, read by the worker after each job.

use std::sync::{Arc, Mutex};

use nox_shield_core::ffi::Wallet;

/// What a state request answers, taken by the worker after each job. A
/// sync or a proof holds the store for minutes; the answer never waits on it.
pub(super) struct Seen {
    pub(super) account: String,
    pub(super) wallet: Arc<Wallet>,
    pub(super) address: Option<String>,
    /// One line a coin, or why the store would not give them.
    pub(super) balances: Result<Vec<String>, String>,
    pub(super) live: bool,
    /// Where a withdrawal pays, read here so asking for it never waits on
    /// the store a proof holds. None for a wallet from a key; an error is
    /// the store's, never taken for that.
    pub(super) fresh: Result<Option<String>, String>,
    /// The account count and the addresses mode 1 may hand out (v1.4.52).
    pub(super) ahead: (u32, Vec<(u32, [u8; 20])>),
    /// The account's history from its store, one line an entry, oldest
    /// first, or why the store would not give it.
    pub(super) history: Result<Vec<String>, String>,
}

pub(super) static SEEN: Mutex<Option<Seen>> = Mutex::new(None);
