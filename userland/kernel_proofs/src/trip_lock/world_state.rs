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

//! The state of one world, and where every search starts.

use super::trip_rows::Rows;
use super::world::P;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(super) struct World {
    pub rows: Rows,
    pub p: [P; 4],
    pub alive: [bool; 4],
    pub rounds: [u8; 4],
    pub nested: [u8; 4],
    pub woken: [bool; 4],
    pub failed: [bool; 4],
    pub stopped: bool,
    pub stop_used: bool,
    pub held: Option<u32>,
    pub reused: bool,
    pub up: bool,
    pub bad: u16,
}

impl World {
    pub(super) fn start(up: bool) -> Self {
        World {
            rows: Rows::new(),
            p: [P::Out; 4],
            alive: [true; 4],
            rounds: [1; 4],
            nested: [0, 1, 0, 1],
            woken: [false; 4],
            failed: [false; 4],
            stopped: false,
            stop_used: false,
            held: None,
            reused: false,
            up,
            bad: 0,
        }
    }
}
