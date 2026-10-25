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

//! The terminal's whole-file reads at their caps: the cap taken, one byte more refused.
//
// The readers (fs slurp and cat, nox read, copy and push, the git store and
// the `<` input) are the real capsule sources, mounted at the module paths
// they have in the capsule so their `super::` and `crate::` names resolve.
// They name `nonos_app_skeleton` and `nonos_libc`, which this target answers
// as itself: the vfs client, mk_getpid, State, Scrollback and the push's PUT
// are host stand-ins (clients.rs, process.rs, term.rs, push_request.rs).

extern crate alloc;
extern crate self as nonos_app_skeleton;
extern crate self as nonos_libc;

#[path = "read_caps_host/clients.rs"]
pub mod clients;
#[path = "read_caps_host/command.rs"]
mod command;
#[path = "read_caps_host/fixtures.rs"]
mod fixtures;
#[path = "read_caps_host/git.rs"]
mod git;
#[path = "read_caps_host/git_tests.rs"]
mod git_tests;
#[path = "read_caps_host/process.rs"]
mod process;
#[path = "read_caps_host/term.rs"]
mod term;

pub use process::mk_getpid;
