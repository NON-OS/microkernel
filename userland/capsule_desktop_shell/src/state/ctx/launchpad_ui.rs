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

//! The Launchpad overlay: whether it is open, its query, page and wheel, and
//! the targets it shows.

pub struct LaunchpadUi {
    /// Whether the full-screen Launchpad overlay is open.
    pub launchpad: bool,
    pub launchpad_query: alloc::string::String,
    pub launchpad_page: usize,
    /// Wheel notches added up toward the next page turn.
    pub launchpad_wheel: crate::state::launchpad_wheel::PageWheel,
    pub launchpad_view: alloc::vec::Vec<crate::render::launchpad::Target>,
}

impl Default for LaunchpadUi {
    fn default() -> Self {
        Self::new()
    }
}

impl LaunchpadUi {
    pub fn new() -> Self {
        LaunchpadUi {
            launchpad: false,
            launchpad_query: alloc::string::String::new(),
            launchpad_page: 0,
            launchpad_wheel: crate::state::launchpad_wheel::PageWheel::default(),
            launchpad_view: alloc::vec::Vec::new(),
        }
    }
}
