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

use alloc::vec::Vec;

// Every command name and alias the shell accepts as a first token, used
// for Tab completion of the command word.
const COMMANDS: &[&[&[u8]]] = &[SHELL, FILES, SYSTEM, TEXT, LATE];

#[path = "complete_words.rs"]
mod complete_words;
#[path = "complete_words_more.rs"]
mod complete_words_more;
use complete_words::{FILES, SHELL};
use complete_words_more::{LATE, SYSTEM, TEXT};
#[path = "complete_takes.rs"]
mod complete_takes;
pub use complete_takes::takes_command_argument;

/// Whether the shell answers to this name, by the same table Tab completes
/// from. One list, so `type` can never claim a command that completion does
/// not offer, or the reverse.
pub fn is_command_name(name: &[u8]) -> bool {
    COMMANDS.iter().any(|group| group.contains(&name))
}

/// Every name that would run: shell commands and installed tools alike.
///
/// One source for completion, for `type`, and for the suggestion made when a
/// name is not found, so all three agree about what exists.
pub fn all_names() -> Vec<&'static [u8]> {
    COMMANDS
        .iter()
        .flat_map(|group| group.iter().copied())
        .chain(crate::command::builtin::tool::TOOLS.iter().map(|(typed, _)| *typed))
        .collect()
}

/// Command names, plus the installed tools.
///
/// The tools are read from their own table rather than copied into `COMMANDS`,
/// because a second list of the same names is a list that will disagree with
/// the first one eventually. A tool that can be run and cannot be completed is
/// a tool nobody finds.
pub(super) fn command_candidates(prefix: &[u8]) -> Vec<&'static [u8]> {
    all_names().into_iter().filter(|c| c.starts_with(prefix)).collect()
}

pub(super) fn common_prefix(cands: &[&[u8]]) -> Vec<u8> {
    let Some(first) = cands.first() else {
        return Vec::new();
    };
    let mut p = first.to_vec();
    for c in &cands[1..] {
        let n = p.iter().zip(c.iter()).take_while(|(a, b)| a == b).count();
        p.truncate(n);
    }
    p
}
