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

//! What each text filter accepts, and the lines a file gives it.
//!
//! The filters run over a pipe's lines (`ls | sort`), over a redirected file
//! (`sort < f`) and over files named on the line (`sort f`). One flag table
//! per filter serves all three, so `help sort` and every way of running it
//! agree on what `-n` means.

use alloc::vec::Vec;

use crate::command::flags::Spec;

#[path = "input_lines.rs"]
mod input_lines;

pub use input_lines::{lines_of, without};

/// The names that read lines rather than files when they follow a `|`.
pub const FILTERS: [&[u8]; 10] =
    [b"grep", b"sort", b"uniq", b"cut", b"nl", b"wc", b"head", b"tail", b"tac", b"rev"];

/// The flags a filter takes.
pub fn spec_for(name: &[u8]) -> Option<Spec<'_>> {
    Some(match name {
        b"grep" => Spec::new(b"grep", b"cinv"),
        b"sort" => Spec::new(b"sort", b"nru"),
        b"uniq" => Spec::new(b"uniq", b"c"),
        b"cut" => Spec::new(b"cut", b"").valued(b"df"),
        b"wc" => Spec::new(b"wc", b"lwc"),
        b"head" => Spec::new(b"head", b"").valued(b"n").numeric(b'n'),
        b"tail" => Spec::new(b"tail", b"").valued(b"n").numeric(b'n'),
        b"nl" => Spec::new(b"nl", b""),
        b"tac" => Spec::new(b"tac", b""),
        b"rev" => Spec::new(b"rev", b""),
        _ => return None,
    })
}

/// What a pipe stage that is not a filter says: it would have dropped the
/// lines piped into it without a word.
pub fn not_a_filter(name: &[u8]) -> Vec<u8> {
    let mut msg = Vec::from(&b"pipe: "[..]);
    msg.extend_from_slice(name);
    msg.extend_from_slice(b" does not read from a pipe; these do:");
    for f in FILTERS {
        msg.push(b' ');
        msg.extend_from_slice(f);
    }
    msg
}
