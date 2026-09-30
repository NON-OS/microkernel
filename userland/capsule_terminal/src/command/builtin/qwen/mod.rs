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

//! `qwen`: talk with a Qwen model on this machine, in this tab. The kernel
//! runs the chosen tier through the Linux personality as a child of this
//! terminal; its output is this screen and its stdin is the keyboard.
//! `qwen window [tier]` instead opens the tier in its own desktop window;
//! `qwen get TIER...` downloads tiers and `qwen tiers` lists them.

mod ask;
mod complete;
mod enter;
mod fetch;
mod fetch_check;
mod fetch_words;
mod help;
mod open;
mod refused;
mod start;
mod statement;
mod tiers;
mod window;

pub use ask::is_line;
pub use complete::words as complete_words;
pub use enter::enter;
pub use help::HELP;
pub use statement::{from_args, misplaced};
