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

//! The options `opt_more` keeps, with Linux's starting values.

#[derive(Clone, Copy)]
pub struct More {
    pub tos: u32,
    pub ttl: u32,
    pub priority: u32,
    pub user_timeout: u32,
    pub quickack: bool,
    pub fastopen: u32,
}

impl Default for More {
    fn default() -> More {
        More { tos: 0, ttl: 64, priority: 0, user_timeout: 0, quickack: true, fastopen: 0 }
    }
}
