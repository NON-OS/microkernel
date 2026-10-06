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

mod default_net;
mod empty_rail;
mod filter_rails;
mod hydrate;
mod live_view;
mod network;
mod new;
mod rail_allowed;
mod record_tx;
mod restore;
mod restore_words;
pub mod shield_log;
pub mod shield_ui;
mod types;

pub use default_net::default_net;
pub use hydrate::hydrate;
pub use live_view::needs_live_data;
pub use network::{forget_live, switch_network};
pub use new::new_state;
pub use record_tx::record_tx;
pub use types::{
    Rail, State, MAX_RAILS, SEND_FIELD_AMOUNT, SEND_FIELD_TO, VIEW_EXPORT, VIEW_HOME,
    VIEW_IMPORT, VIEW_NOX, VIEW_RECEIVE, VIEW_RECOVER, VIEW_SEND, VIEW_SETTINGS, VIEW_SHIELD,
    VIEW_SWAP, VIEW_ACCOUNTS, WORDS_HELD, WORDS_NONE, WORDS_UNREAD,
};
