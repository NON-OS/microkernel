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

//! The state a window starts from, before the market has answered anything.

use alloc::vec::Vec;

use super::super::state::{State, Tab};

impl State {
    pub fn new() -> State {
        State {
            listings: Vec::new(),
            tab: Tab::All,
            cursor: 0,
            scroll: 0,
            rows: 1,
            fb_w: 0,
            fb_h: 0,
            trouble: None,
            asked: None,
            orders: Default::default(),
            proofs: Default::default(),
            search: super::super::search::Search::default(),
            port: 0,
            loaded: false,
            memory: None,
            room: crate::need::Room::Memory,
            route: crate::net::Route::Down(""),
            fetch: None,
            fetch_at_ms: 0,
        }
    }
}
