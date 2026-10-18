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

/*
 * Opening the shield for this account. If the service already holds this
 * account's store open, the wallet only reads it again. Otherwise the
 * keyring gives the recovery words, which go straight to the service and
 * are zeroed here; an account imported from a key has no words, and its
 * key goes the same way. The service then checks the account it derived
 * is this one, and refuses to open anything else.
 */

use alloc::string::String;

use shield_wire::*;

use nonos_libc::mk_uptime_ms;

use super::client::{call, NO_ANSWER};
use crate::wallet::ipc::{export_secret, shield_words};
use crate::wallet::screen::receive_address::address_hex;
use crate::wallet::state::{State, WORDS_NONE, WORDS_UNREAD};

#[path = "open_ask.rs"]
mod open_ask;
#[path = "open_ensure.rs"]
mod open_ensure;
#[path = "open_lock.rs"]
mod open_lock;
#[path = "open_retry.rs"]
mod open_retry;
#[path = "open_said.rs"]
mod open_said;
#[path = "open_words.rs"]
mod open_words;
pub use open_ask::open;
pub use open_ensure::ensure;
use open_ensure::{held, probe_then_open};
pub use open_lock::{forget_account, forget_network, lock};
use open_retry::{answered, keyring_late, later, slow};
pub use open_retry::{restarted, retry_due, retrying};
pub use open_said::{here, opening_said, ELSEWHERE};
use open_words::from_words;

const NO_ACCOUNT: &str = "Make or restore an account first. The shield opens from its words.";
const NO_WORDS: &str = "The keyring would not hand this account's words to the shield.";
const NO_KEYRING: i32 = -2;
/* The keyring took longer than the call allows, as while it unseals. */
const KEYRING_SLOW: i32 = -11;
const KEYRING_LATE: &str = "The keyring did not answer in time. The shield is asked to open \
     again by itself in a few seconds.";
const WORDS_UNREAD_YET: &str = "This account's recovery words have not been read from this \
     machine yet. The shield opens from them once they are, and is asked again by itself.";
