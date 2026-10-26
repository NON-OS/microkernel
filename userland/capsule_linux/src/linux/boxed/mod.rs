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

//! `linux --box <preset> <program>`: a program run in a named box, and the
//! report of everything the box refused it.

mod allowed;
mod app_line;
mod connected;
mod dests;
mod end;
mod finish;
mod folders;
mod frames;
mod hash;
mod items;
mod kind;
mod learn_folders;
mod learn_report;
mod learned;
mod learned_text;
mod line;
mod net_report;
mod notes;
mod preset;
mod report;
mod run_line;
mod run_tally;
mod state;
mod tally;
mod terminal_run;

pub use finish::{finish, not_started};
pub use frames::key_of;
pub use kind::{of_needs, Kind};
pub use notes::{armed_hash, call_begins, connected, learning, may_write, note, tried, wrote};
pub use run_tally::report as run_report;
pub use state::{arm, exec_program, narrow};
pub use terminal_run::for_terminal;
