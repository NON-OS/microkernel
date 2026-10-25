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

//! The layout solver and row helpers, with `types` mirrored as plain data.

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

        #[derive(Clone, Copy)]
        pub struct Chrome {
            pub titlebar_h: u32,
            pub tabstrip_h: u32,
            pub body_pad_top: u32,
            pub footer_h: u32,
            pub text_left: u32,
            pub row_h: u32,
        }

        #[derive(Clone, Copy)]
        pub struct Rails {
            pub left: u32,
        }

        #[derive(Clone, Copy)]
        pub struct Layout {
            pub left_rail: Rect,
            pub body: Rect,
            pub input: Rect,
            pub footer: Rect,
        }
    }

    pub mod limits;

    pub mod compute;
}

#[path = "../../../capsule_terminal/src/layout/rows.rs"]
pub mod rows;
