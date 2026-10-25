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

//! The market's boot catalogue read, driven through boot_index::load.
//
// boot_index.rs, ingest/error.rs and libc's debug.rs are the shipped sources.
// boot_index names `nonos_libc` and `nonos_app_skeleton`, which this crate
// answers as itself. Host stand-ins (boot_index_host/) are the baseline, the
// ingest step, the store, the verifier bound, the syscall under mk_debug, the
// capsule's pid and the vfs client. A swap of the boot_index mount path runs
// these on another file.

extern crate alloc;
extern crate self as nonos_app_skeleton;
extern crate self as nonos_libc;

#[path = "../../capsule_market/src/boot_index.rs"]
mod boot_index;
#[path = "../../libc/src/debug.rs"]
mod debug;

#[path = "boot_index_host/baseline.rs"]
mod baseline;
#[path = "boot_index_host/clients.rs"]
pub mod clients;
#[path = "boot_index_host/fixture.rs"]
mod fixture;
#[path = "boot_index_host/ingest.rs"]
mod ingest;
#[path = "boot_index_host/process.rs"]
mod process;
#[path = "boot_index_host/store.rs"]
mod store;
#[path = "boot_index_host/syscall.rs"]
mod syscall;
#[path = "boot_index_host/verify.rs"]
mod verify;

#[path = "boot_index_host/read_cap_tests.rs"]
mod read_cap_tests;
#[path = "boot_index_host/reason_tests.rs"]
mod reason_tests;

pub use debug::mk_debug;
pub use process::mk_getpid;
