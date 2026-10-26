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

//! The installer's load and pkg handlers, driven through their frames.
//
// The handlers, discover.rs and the request wire are the real installer
// sources; pkg_verify parses with the real nonos-pack-core and digests with
// blake3. They name `nonos_libc` and `nonos_app_skeleton`, which this crate
// answers as itself with libc's real capsule_load, capsule_verify and lookup.
// Host stand-ins (mounts.rs, support/) are only the syscalls under those, the
// installer's own pid and the vfs client. A swap of the mount paths runs these
// on other files.

extern crate alloc;
extern crate self as nonos_app_skeleton;
extern crate self as nonos_libc;

#[path = "../../capsule_installer/src/server/handlers"]
mod handlers {
    pub mod load_by_name;
    pub mod load_outcome;
    pub mod pkg_body;
    pub mod pkg_commit;
    pub mod pkg_paths;
    pub mod pkg_query;
    pub mod pkg_remove;
    pub mod pkg_verify;
}
#[path = "../../capsule_installer/src/server/discover.rs"]
mod discover;
#[path = "../../capsule_installer/src/protocol/mod.rs"]
mod protocol;

#[path = "handlers_host/mounts.rs"]
mod mounts;

#[path = "handlers_host/load_by_name_read_tests.rs"]
mod load_by_name_read_tests;
#[path = "handlers_host/load_by_name_tests.rs"]
mod load_by_name_tests;
#[path = "handlers_host/retired_tests.rs"]
mod retired_tests;
#[path = "handlers_host/gate_tests.rs"]
mod gate_tests;
#[path = "handlers_host/own_caps_glue_tests.rs"]
mod own_caps_glue_tests;
#[path = "handlers_host/own_caps_tests.rs"]
mod own_caps_tests;
#[path = "handlers_host/pkg_commit_tests.rs"]
mod pkg_commit_tests;
#[path = "handlers_host/pkg_installed_tests.rs"]
mod pkg_installed_tests;
#[path = "handlers_host/pkg_query_tests.rs"]
mod pkg_query_tests;
#[path = "handlers_host/protocol_tests.rs"]
mod protocol_tests;

use mounts::{alloc_watch, clients, fixtures, frame, packages, pkg_frame};
pub use mounts::{mk_capsule_load, mk_capsule_verify, mk_getpid, mk_service_lookup};
pub use mounts::{mk_caps_of, CAPS_OF_APP_INSTALL, CAPS_OF_SPAWN_WINDOW};
use mounts::{process, server, services, syscall, verify};
pub use mounts::{CapsuleLoadRequest, CapsuleVerifyRequest, CapsuleVerifySummary};
