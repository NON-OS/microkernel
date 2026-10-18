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

//! The rail formatters and metric types, mounted by path.

#[path = "../../../capsule_terminal/src/term"]
pub mod term {
    pub mod util {
        pub mod copy_into;
        pub mod format_u64;

        pub use copy_into::copy_into;
        pub use format_u64::format_u64;
    }
}

#[path = "../../../capsule_terminal/src/paint"]
pub mod paint {
    pub mod rail_fmt;

    pub mod rail_metric;

    pub mod rail_addr;
}

#[path = "../../../capsule_terminal/src/rail"]
pub mod rail {
    pub mod disk;
    pub mod mem;
    pub mod metrics;
    pub mod net;
    pub mod value;
}
