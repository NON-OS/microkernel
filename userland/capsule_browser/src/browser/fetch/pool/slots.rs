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

//! The sub-resource fetches of a page, side by side. One slot used to serve
//! the page and everything it asked for, one after another on a new
//! connection each: thirty-one resources took thirty-three seconds.

use alloc::string::String;
use alloc::vec::Vec;

use super::idle::Idle;
use crate::browser::fetch::types::Fetch;

#[derive(Default)]
pub struct Pool {
    pub live: Vec<Fetch>,
    pub idle: Vec<Idle>,
    /* Script bodies that arrived ahead of an earlier script, by order. */
    pub held: Vec<(u32, Vec<u8>)>,
    /* The order the next script launched takes, and the next to run. */
    pub next_order: u32,
    pub run_order: u32,
    /* Image redirects waiting for a slot: target, key, hops so far. */
    pub redirects: Vec<(String, String, u8)>,
}

impl Pool {
    pub const fn new() -> Pool {
        Pool {
            live: Vec::new(),
            idle: Vec::new(),
            held: Vec::new(),
            next_order: 0,
            run_order: 0,
            redirects: Vec::new(),
        }
    }

    /// Work in flight or waiting its turn.
    pub fn busy(&self) -> bool {
        !self.live.is_empty() || !self.held.is_empty() || !self.redirects.is_empty()
    }
}
