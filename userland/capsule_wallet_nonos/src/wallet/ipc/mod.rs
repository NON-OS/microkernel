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

mod address;
mod call;
mod constants;
mod decode_rails;
mod derive;
mod export;
mod forget;
mod generate_hd;
mod import;
mod lookup_keyring;
mod lookup_self;
mod ops;
mod push_word;
mod read_rails;
mod recover;
mod shield_words;
mod sign_tx;
mod words_vault;

pub use address::wallet_address;
pub use call::keyring_call;
pub use constants::{HDR_LEN, OP_VAULT_OPEN, OP_VAULT_SEAL};
pub use decode_rails::decode_rails;
pub use derive::derive_account;
pub use export::export_secret;
pub use forget::forget_key;
pub use generate_hd::generate_wallet_hd;
pub use import::import_wallet;
pub use lookup_keyring::lookup_keyring;
pub use lookup_self::lookup_self_pid;
pub use read_rails::read_rails;
pub use recover::recover_wallet;
pub use shield_words::shield_words;
pub use sign_tx::{sign_tx, TxRequest};
pub use words_vault::{open_words, seal_words, WORDS_BLOB_LEN};
