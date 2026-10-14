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
//! The key screens: generating, importing, restoring, exporting and backing up a wallet.

pub(in crate::wallet::event) mod backup;
pub(in crate::wallet::event) mod custody_guard;
pub(in crate::wallet::event) mod export_key;
pub(in crate::wallet::event) mod generate;
pub(in crate::wallet::event) mod generate_draw;
pub(in crate::wallet::event) mod import;
pub(in crate::wallet::event) mod import_submit;
pub(in crate::wallet::event) mod keyring_says;
pub(in crate::wallet::event) mod recover;
pub(in crate::wallet::event) mod recover_input;
pub(in crate::wallet::event) mod recover_keyring;
pub(in crate::wallet::event) mod recover_submit;
pub(in crate::wallet::event) mod replace_rule;
