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

//! Layout, term and rail modules, mirrored or mounted by path.

#[path = "../../src/layout"]
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

#[path = "../../src/term"]
pub mod term {
    pub mod util {
        pub mod copy_into;
        pub mod format_u64;

        pub use copy_into::copy_into;
        pub use format_u64::format_u64;
    }
}

#[path = "../../src/rail"]
pub mod rail {
    pub mod disk;
    pub mod disk_decode;
    pub mod mem;
    pub mod metrics;
    pub mod net;
    pub mod net_decode;
    pub mod value;
}
