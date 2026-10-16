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

//! The rail geometry modules, with `Rect` and the TTF metrics mirrored.

#[path = "../../../capsule_terminal/src/layout"]
pub mod layout {
    pub mod types {
        #[derive(Clone, Copy, PartialEq, Debug, Default)]
        pub struct Rect {
            pub x: u32,
            pub y: u32,
            pub w: u32,
            pub h: u32,
        }
    }

    pub mod rows;

    pub use types::Rect;
}

#[path = "../../../capsule_terminal/src/paint"]
pub mod paint {
    pub mod rail_text {
        pub const RAIL_PAD: u32 = 12;
        pub const RAIL_GAP: u32 = 8;
        pub const BAR_H: u32 = 3;

        pub fn lh() -> u32 {
            22
        }
    }

    pub mod rail_band;

    pub mod rail_row;

    pub mod rail_left_geom;

    pub mod rail_geom;

    pub mod rail_scroll;
}
