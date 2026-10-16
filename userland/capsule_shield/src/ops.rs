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

//! Each call the wallet makes, on the phones' wallet object.

use shield_wire::Values;

mod account;
mod args;
mod folder;
mod follow;
mod followed;
mod fresh_seen;
mod held;
mod lock;
mod open;
mod open_finish;
mod open_from;
mod prove;
mod public;
mod publish;
mod quote;
mod remember;
mod review;
mod seen;
mod spend;
mod state;
mod state_history;
mod sync;

pub use args::coin;
pub use follow::follow;
pub use fresh_seen::fresh_address;
pub(crate) use fresh_seen::seen_ahead;
pub(crate) use held::wallet;
pub use lock::lock;
pub use open_from::{open_key, open_words};
pub use prove::{cancel, progress};
pub use public::{confirm, take_back};
pub use quote::quote;
pub use remember::remember;
pub use review::{review_self_settle, review_shield};
pub use spend::{send, withdraw};
pub use state::state;
pub use sync::sync;

pub type Answer = Result<Values, String>;

fn refuse<T>(why: impl core::fmt::Display) -> Result<T, String> {
    Err(why.to_string())
}
