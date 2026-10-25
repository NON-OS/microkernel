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

//! The libc sources and host stand-ins the handlers_host target mounts, named at its root.

#[path = "../support/alloc_watch.rs"]
pub mod alloc_watch;
#[path = "../support/caps_of.rs"]
pub mod caps_of;
#[path = "../../../libc/src/capsule_load.rs"]
mod capsule_load;
#[path = "../../../libc/src/capsule_verify.rs"]
mod capsule_verify;
#[path = "../support/pkg_clients.rs"]
pub mod clients;
#[path = "../support/fixtures.rs"]
pub mod fixtures;
#[path = "../../../capsule_installer/src/server/gate.rs"]
pub mod gate;
#[path = "frame.rs"]
pub mod frame;
#[path = "../../../libc/src/ipc/lookup.rs"]
mod lookup;
#[path = "../support/packages.rs"]
pub mod packages;
#[path = "pkg_frame.rs"]
pub mod pkg_frame;
#[path = "../support/process.rs"]
pub mod process;
#[path = "../support/services.rs"]
pub mod services;
#[path = "../support/pkg_syscall.rs"]
pub mod syscall;
#[path = "../support/verify.rs"]
pub mod verify;

pub use capsule_load::{mk_capsule_load, CapsuleLoadRequest};
pub use capsule_verify::{mk_capsule_verify, CapsuleVerifyRequest, CapsuleVerifySummary};
pub use caps_of::{mk_caps_of, CAPS_OF_APP_INSTALL, CAPS_OF_SPAWN_WINDOW};
pub use lookup::mk_service_lookup;
pub use process::mk_getpid;
