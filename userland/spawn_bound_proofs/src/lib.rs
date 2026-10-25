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

//! The kernel's `from_vfs/load/bound.rs` and `from_vfs/error.rs`, mounted at
//! their module paths over the kernel's own capability table, beside the
//! verified spawn's capability arithmetic. What they reach beyond that
//! stands in (`stand_in`).

#![cfg(test)]

extern crate alloc;

mod stand_in;
mod tests;

pub use stand_in::{process, security, sys, syscall};

#[path = "../../../src/capabilities/types"]
pub mod capabilities {
    mod as_str;
    pub mod defs;
    mod guard;
    mod table;
    pub use defs::Capability;
}

#[path = "../../../src/process/core/table/ambient.rs"]
mod ambient;
#[path = "../../../src/security/capsule_manifest/verify/caps_bits.rs"]
mod spawn_caps;

#[path = "../../../src/kernel_core"]
pub mod kernel_core {
    pub mod process_spawn {
        pub mod capsule_spawn {
            pub use crate::stand_in::{AttestedParent, SpawnError};
            pub(crate) use from_vfs::load::bound::{installed as child_caps, missing as beyond_parent};
            pub mod from_vfs {
                pub mod error;
                pub mod load {
                    pub mod asked;
                    pub mod bound;
                }
            }
        }
    }
}

#[path = "../../../src/userspace/tool_capsules/bound.rs"]
mod tool_bound;
