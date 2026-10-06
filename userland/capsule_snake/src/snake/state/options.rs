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

use super::mode::Mode;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Options {
    pub obstacles: bool,
    pub wrap: bool,
    pub powerups: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

impl Options {
    pub fn new() -> Self {
        Options { obstacles: true, wrap: false, powerups: true }
    }

    // Zen forces wrapping on and Classic forces it off; otherwise the toggle
    // is the whole story. Nothing else is coupled.
    pub fn wraps(&self, mode: Mode) -> bool {
        if mode.forces_wrap() {
            return true;
        }
        if mode.hard_walls() {
            return false;
        }
        self.wrap
    }

    // Zen and Classic decide wrapping themselves, so the switch is locked there.
    pub fn wrap_locked(mode: Mode) -> bool {
        mode.forces_wrap() || mode.hard_walls()
    }

    // Flip the rule at `index` (Obstacles, Wrap edges, Power-ups, the order of
    // `setup_geom_rows::TOGGLE_LABELS`). False when nothing changed: a locked
    // switch, or no switch at all.
    pub fn flip(&mut self, index: usize, mode: Mode) -> bool {
        match index {
            0 => self.obstacles = !self.obstacles,
            1 if !Options::wrap_locked(mode) => self.wrap = !self.wrap,
            2 => self.powerups = !self.powerups,
            _ => return false,
        }
        true
    }
}
