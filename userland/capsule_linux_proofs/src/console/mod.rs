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

//! The console's input queue, mounted as the capsule's own modules so each
//! finds the others where it does in the capsule, and the run request's
//! parser, which decides whether a run starts on a terminal.

#[path = "../../../capsule_linux/src/linux/console/queue.rs"]
pub mod queue;
#[path = "../../../capsule_linux/src/linux/console/queue_piece.rs"]
pub mod queue_piece;
#[path = "../../../capsule_linux/src/linux/console/queue_take.rs"]
pub mod queue_take;
#[path = "../../../capsule_linux/src/linux/console/tty_rules.rs"]
pub mod tty_rules;
#[cfg(test)]
mod tty_rules_tests;
#[path = "../../../capsule_linux/src/linux/console/wipe.rs"]
pub mod wipe;
#[path = "../../../capsule_linux/src/linux/console/winsize.rs"]
pub mod winsize;
#[path = "../../../capsule_linux/src/linux/run_mode.rs"]
pub mod run_mode;
