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

//! AgentProof box policies as files: the `nonos-box 1` format (format/),
//! what a file compiles to (compile.rs) and what it allows in plain words
//! (explain.rs). docs/agentproof/policy-format.md is the format's page.

#![no_std]

extern crate alloc;

pub mod boxed;
pub mod cli;
pub mod compile;
pub mod explain;
pub mod format;

pub use compile::{compile, Compiled};
pub use format::{parse, Base, Error, Policy};
