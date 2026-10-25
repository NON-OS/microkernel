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

//! The marketplace window: the catalogue the market capsule serves, the three
//! namespaces it carries, and why any one listing can or cannot be installed
//! on this machine.

mod app;
mod asked;
mod asked_label;
mod channel_held;
mod event;
mod event_actions;
mod event_click;
mod event_keys;
mod event_rows;
mod event_search;
mod event_tab;
mod fetch_status;
mod fill;
mod fill_order;
mod fill_step;
pub(crate) mod install;
mod install_direct;
mod install_primary;
mod install_remove;
mod listing;
mod machine;
mod manifest;
pub mod market;
mod meter_call;
mod model_card;
mod model_weights;
mod next_step;
mod order_line;
mod order_watch;
mod orders;
mod payee;
mod poll;
pub mod price;
mod progress;
mod progress_text;
mod purchase;
mod purchase_said;
mod reasons;
mod refused;
mod route_offer;
pub mod search;
mod source;
mod state;
mod state_move;
mod state_ops;
mod state_refresh;
mod state_select;
mod state_window;
mod tab;
mod theme;
mod tier_fit;
mod ui;
mod verdict;

pub use app::Store;
