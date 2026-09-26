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
mod event;
mod consent;
mod event_actions;
mod event_click;
mod event_keys;
mod event_rows;
mod event_search;
mod event_tab;
mod install;
mod listing;
pub mod market;
mod manifest;
pub mod search;
mod state;
mod state_move;
mod state_refresh;
mod state_select;
mod state_window;
mod tab;
mod state_ops;
mod theme;
mod ui;
mod verdict;

pub use app::Store;
