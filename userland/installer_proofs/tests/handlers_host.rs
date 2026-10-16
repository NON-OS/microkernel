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

//! The installer's load_store and load_by_name handlers, driven through their frames.
//
// The handlers (with their layout and artifacts parts) and the request wire
// are the real installer sources. They name `nonos_libc` and
// `nonos_app_skeleton`, which this crate answers as itself: the real
// CapsuleLoadRequest and mk_capsule_load are mounted from libc, and only the
// syscall under them, the installer's own pid and the vfs read are host
// stand-ins (support/). A swap of the handlers path runs these on other files.

extern crate alloc;
extern crate self as nonos_app_skeleton;
extern crate self as nonos_libc;

#[path = "../../capsule_installer/src/server/handlers"]
mod handlers {
    pub mod load_by_name;
    pub mod load_store;
}
#[path = "../../capsule_installer/src/protocol/mod.rs"]
mod protocol;

#[path = "support/alloc_watch.rs"]
mod alloc_watch;
#[path = "../../libc/src/capsule_load.rs"]
mod capsule_load;
#[path = "support/clients.rs"]
pub mod clients;
#[path = "support/fixtures.rs"]
mod fixtures;
#[path = "handlers_host/frame.rs"]
mod frame;
#[path = "support/payloads.rs"]
mod payloads;
#[path = "support/process.rs"]
mod process;
#[path = "support/syscall.rs"]
mod syscall;

#[path = "handlers_host/load_by_name_read_tests.rs"]
mod load_by_name_read_tests;
#[path = "handlers_host/load_by_name_tests.rs"]
mod load_by_name_tests;
#[path = "handlers_host/load_store_accept_tests.rs"]
mod load_store_accept_tests;
#[path = "handlers_host/load_store_tests.rs"]
mod load_store_tests;
#[path = "handlers_host/protocol_tests.rs"]
mod protocol_tests;

pub use capsule_load::{mk_capsule_load, CapsuleLoadRequest};
pub use process::mk_getpid;
