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
//! This phrase's accounts: which are in use and which is open, its public
//! address, where its words stand, and which address Receive shows.

use alloc::vec::Vec;

use crate::wallet::accounts::Account;

pub struct AccountSet {
    pub address: [u8; 20],
    pub address_ready: bool,
    /// The accounts of this phrase in use, account 0 first, and which one is
    /// open. Empty until a wallet exists.
    pub list: Vec<Account>,
    pub open: u8,
    /// The account list read at boot, while some of its accounts are not
    /// derived yet: how many, and which was open.
    pub owed: Option<(u8, u8)>,
    /// When the owed accounts are asked for again (uptime ms).
    pub retry_at: i64,
    /// WORDS_*: the shield opens from the words, and from the key only for
    /// a wallet known to have come from one.
    pub words: u8,
    /// Receive shows the private nox1 address, else the public 0x one.
    pub receive_private: bool,
}

impl AccountSet {
    pub fn new() -> Self {
        Self {
            address: [0; 20],
            address_ready: false,
            list: Vec::new(),
            open: 0,
            owed: None,
            retry_at: 0,
            words: super::types::WORDS_UNREAD,
            /* Mainnet is picked at start, and it has no shield pool. */
            receive_private: false,
        }
    }
}
