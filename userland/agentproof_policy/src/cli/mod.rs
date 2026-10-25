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

//! What `agentproof policy` answers, as a pure function of its words and the
//! file's bytes, so its tests drive what the binary prints and exits with.

mod answer;

use alloc::string::String;
use alloc::vec::Vec;

pub use answer::{answer, unreadable, usage};

pub const USAGE: &str = "usage: agentproof policy check|word|hash <file>
  check  say what the policy allows, in plain words, and its box word and hash
  word   print the box word only, as `linux --box` takes it
  hash   print the box's policy hash only, as the run's report shows it
The format is `nonos-box 1`: docs/agentproof/policy-format.md.";

/// Exit 0, a policy; 1, a file refused; 2, words or a file it cannot use.
pub struct Answer {
    pub code: u8,
    pub out: Vec<String>,
    pub err: Vec<String>,
}

pub enum Ask<'a> {
    /// A verb on a file, with its name for messages.
    Policy(&'a str, &'a str),
    Help,
    Version,
    Usage,
}

pub fn ask<'a>(words: &[&'a str]) -> Ask<'a> {
    match words {
        ["policy", verb @ ("check" | "word" | "hash"), file] => Ask::Policy(verb, file),
        ["--help" | "-h" | "help"] => Ask::Help,
        ["--version"] => Ask::Version,
        _ => Ask::Usage,
    }
}
