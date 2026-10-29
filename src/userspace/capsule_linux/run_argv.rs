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

//! The package a run role's argv names, pure. Held in kernel_proofs.
//!
//! init spawns a run as `run <package>`, or, for a served launch that gave
//! its run a word (served_word.rs), as `run <package> word <word>`. Either
//! names the package, so the run may ask to start again with the network
//! (MkLinuxRunNet) as itself. No other shape names one: a process whose
//! words are not init's is the run of no package.

use alloc::string::String;

/// The package `argv` was spawned to run, if it is a run's.
pub(super) fn package_of(argv: &[String]) -> Option<&String> {
    match argv {
        [verb, package] if verb == "run" => Some(package),
        [verb, package, mark, _] if verb == "run" && mark == "word" => Some(package),
        _ => None,
    }
}
