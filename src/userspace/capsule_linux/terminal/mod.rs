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
//! The personality started by a terminal, as that terminal's own child, to
//! run a Qwen tier in the terminal rather than in a window. A small fixed set
//! of slots, each with endpoints of its own, holds these runs; the process in
//! a slot is private from its first instruction and ends with its terminal.

mod admit;
mod exit;
mod held;
mod roles;
mod run;
mod slots;
mod tier;

pub use admit::{admit_terminal_run, is_private_run};
pub use exit::{end_terminal_runs_of, terminal_run_gone};
pub use run::run_tier_for_caller;
