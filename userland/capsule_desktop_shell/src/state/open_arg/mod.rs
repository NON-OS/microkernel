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

//! What an app's OP_TAKE_OPEN_ARG answer can hold, kept apart from the IPC so
//! it is proven on the host (desktop_proofs open_arg_tests).
//!
//! Two things travel there, and they cannot be mistaken for each other:
//!
//! - a path, for the editor, Files, the viewer and the players. It always
//!   starts with '/': OP_OPEN_WITH refuses anything else, and the desktop's
//!   own icons hand full paths.
//! - a command line, for the Terminal only, from a Launchpad tool tile:
//!   `run:<line>` to run it as typed, or `type:<line>` to leave it on the
//!   prompt for the person to finish. No path starts with either word, and
//!   the Terminal (term/handed/parse.rs) runs nothing that lacks one.
//!
//! The shell keeps the command apart from the paths (`Context::pending_command`)
//! and answers it only to a window of the Terminal, so no other process can
//! put a command there: OP_OPEN_WITH, the one way another process leaves an
//! argument, can only leave a path.

pub mod command;
pub mod opener;
pub mod pending;

pub use command::tool_command;
pub use opener::opens_files;
pub use pending::{is_path, still_wanted, PendingCommand, TERMINAL};
