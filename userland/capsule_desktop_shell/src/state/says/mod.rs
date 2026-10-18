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

//! The words the shell's toasts use when something did not happen, kept apart
//! from the IPC so each is proven on the host (desktop_proofs says_tests). A
//! toast holds `TOAST_TEXT_MAX` bytes; a line built here is cut on its name,
//! never on the part that says what went wrong.

mod launch;
mod refusals;

pub use launch::{named, not_opened, NOTHING_REFUSED, NOT_ASKED, NO_WINDOW, OFF_AT_SETUP};
pub use refusals::{launchpad_empty, package, vfs, NO_REPLY};
