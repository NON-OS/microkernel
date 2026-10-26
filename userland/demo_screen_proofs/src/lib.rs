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

//! The demo's screens drawn on the host through their own paint code. About
//! is mounted whole from capsule_about by `#[path]`; `libc_shim` answers its
//! kernel calls from one fixed machine. The terminal's modules are mounted
//! at the crate root, where its files name each other. tests/screens frames
//! each app as app_skeleton frames a window, at the size it opens at on each
//! display mode the recording may use.

extern crate alloc;

pub mod desk;

#[path = "../../capsule_about/src/about/mod.rs"]
pub mod about;

// The terminal's and the desktop shell's modules, mounted at the crate root
// where their files name each other (crate::paint, crate::state, ...).
include!("mount_terminal.rs");
include!("mount_shell.rs");
