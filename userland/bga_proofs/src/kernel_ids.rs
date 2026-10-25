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

//! The two kernel answers the runtime name rule (register_allowed.rs) asks
//! beside the registry: the group a pid belongs to, and whether that group
//! was spawned from the system's own namespace (L57, L61).

pub mod hardware {
    pub mod broker {
        /// Each capsule here is its own group.
        pub fn holder_of(pid: u32) -> u32 {
            pid
        }
    }
}

pub mod process {
    pub mod core {
        pub mod spawn_tier {
            use std::cell::Cell;

            thread_local! {
                static SYSTEM: Cell<bool> = const { Cell::new(true) };
            }

            /// The system's own, unless a proof says a publisher's.
            pub fn is_system(_pid: u32) -> bool {
                SYSTEM.with(Cell::get)
            }

            pub fn set_system(system: bool) {
                SYSTEM.with(|s| s.set(system));
            }
        }
    }
}
