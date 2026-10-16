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

//! The load handlers' parts on their own: Layout::parse and the Artifacts reader.
//
// layout.rs and artifacts.rs are mounted from the installer as they ship.
// artifacts.rs names `nonos_libc` and `nonos_app_skeleton`, which this crate
// answers as itself, with the real CapsuleLoadRequest and mk_capsule_load
// from libc and host stand-ins for the syscall and the vfs read (support/).

extern crate alloc;
extern crate self as nonos_app_skeleton;
extern crate self as nonos_libc;

#[path = "../../capsule_installer/src/server/handlers/load_by_name/artifacts.rs"]
mod artifacts;
#[path = "../../capsule_installer/src/server/handlers/load_store/layout.rs"]
mod layout;

#[path = "../../libc/src/capsule_load.rs"]
mod capsule_load;
#[path = "support/clients.rs"]
pub mod clients;
#[path = "support/fixtures.rs"]
mod fixtures;
#[path = "support/payloads.rs"]
mod payloads;
#[path = "support/syscall.rs"]
mod syscall;

#[path = "parts_host/artifacts_tests.rs"]
mod artifacts_tests;
#[path = "parts_host/layout_tests.rs"]
mod layout_tests;

pub use capsule_load::{mk_capsule_load, CapsuleLoadRequest};
