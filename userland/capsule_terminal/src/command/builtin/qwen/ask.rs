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

//! A line that starts with `qwen`: an optional tier, then the question,
//! taken byte for byte. The shell's parser never reads it, so an apostrophe,
//! a `&`, a `;` or a `$` in a question stays part of the question, and a
//! question may run to any number of words.

use alloc::vec::Vec;

use super::{chosen::chosen, tiers::TIERS};
use crate::term::util::is_space;

pub struct Ask<'a> {
    /// The tier word, when one was typed.
    pub tier: Option<&'static [u8]>,
    /// Everything after the command and the tier, trimmed.
    pub question: &'a [u8],
}

impl Ask<'_> {
    pub fn tier(&self) -> &'static [u8] {
        self.tier.unwrap_or_else(chosen)
    }

    /// What history and the job list keep: the command and the tier as
    /// typed. The question is never kept.
    pub fn recorded(&self) -> Vec<u8> {
        let mut line = b"qwen".to_vec();
        if let Some(tier) = self.tier {
            line.push(b' ');
            line.extend_from_slice(tier);
        }
        line
    }
}

/// Whether `qwen` is the first word of `line`. Such a line is taken as
/// typed: not even a `!` in it is read as history.
pub fn is_line(line: &[u8]) -> bool {
    parse(line).is_some()
}

/// The line as a question to Qwen, or `None` when `qwen` is not its first
/// word.
pub fn parse(line: &[u8]) -> Option<Ask<'_>> {
    let rest = trim(line).strip_prefix(b"qwen")?;
    if rest.first().is_some_and(|&b| !is_space(b)) {
        return None;
    }
    let rest = trim(rest);
    let end = rest.iter().position(|&b| is_space(b)).unwrap_or(rest.len());
    let tier = TIERS.iter().copied().find(|t| *t == &rest[..end]);
    let question = if tier.is_some() { trim(&rest[end..]) } else { rest };
    Some(Ask { tier, question })
}

fn trim(bytes: &[u8]) -> &[u8] {
    let start = bytes.iter().position(|&b| !is_space(b)).unwrap_or(bytes.len());
    let end = bytes.iter().rposition(|&b| !is_space(b)).map_or(start, |i| i + 1);
    &bytes[start..end]
}
