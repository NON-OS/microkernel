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
//! The session's own aliases, and `type` (also `which`), which says what a
//! name will run. An alias replaces the first word of a command when it
//! runs, once and never through another alias, so no alias can loop. Its
//! name is a variable's name and its body is held to `MAX_VALUE` bytes, and
//! a session to `MAX_ALIASES` of them, as the variables are.

use super::script::is_name;
use super::table::NAMES;
use super::{Session, MAX_VALUE};
use crate::out::ShellOut;
use alloc::vec::Vec;

/// The most aliases one session may hold.
pub const MAX_ALIASES: usize = 64;
/// Said when an alias would pass either bound; nothing changes.
pub const ALIAS_REFUSED: &[u8] =
    b"alias: an alias is held to 1024 bytes and a session to 64 of them";

/// The words the shell reads as control flow before anything runs.
pub const CONTROL: &[&[u8]] =
    &[b"if", b"then", b"else", b"fi", b"for", b"in", b"do", b"done", b"while"];

/// The filters a command's lines may pass through after `|`.
pub const FILTERS: &[&[u8]] =
    &[b"grep", b"sort", b"uniq", b"cut", b"nl", b"wc", b"head", b"tail", b"tac", b"rev"];

/// `alias` lists the aliases; `alias NAME WORD...` sets one.
pub(super) fn alias(s: &mut Session, out: &mut dyn ShellOut, w: &[&[u8]]) -> bool {
    match w {
        [_] => {
            if s.aliases.is_empty() {
                out.writeln(b"(no aliases)");
            }
            for (n, v) in &s.aliases {
                out.writeln(&[&n[..], b" = ", &v[..]].concat());
            }
            true
        }
        [_, name, body @ ..] if is_name(name) && !body.is_empty() => {
            let set = s.set_alias(name, &body.join(&b' '));
            if !set {
                out.writeln(ALIAS_REFUSED);
            }
            set
        }
        _ => {
            out.writeln(b"usage: alias [NAME WORD...]");
            false
        }
    }
}

/// `unalias NAME` removes an alias; removing one that is not set succeeds.
pub(super) fn unalias(s: &mut Session, out: &mut dyn ShellOut, w: &[&[u8]]) -> bool {
    match w {
        [_, name] => {
            s.aliases.retain(|(n, _)| n != name);
            true
        }
        _ => {
            out.writeln(b"usage: unalias NAME");
            false
        }
    }
}

/// `type NAME...` says, for each name, what a command of that name runs,
/// in the order the session looks: an alias first, then a builtin. Control
/// words and filters are named as such. Fails if any name runs nothing.
pub(super) fn kind(s: &Session, out: &mut dyn ShellOut, w: &[&[u8]]) -> bool {
    let Some(names) = w.get(1..).filter(|n| !n.is_empty()) else {
        out.writeln(b"usage: type NAME...");
        return false;
    };
    let mut found = true;
    for name in names {
        let alias = s.aliases.iter().find(|(n, _)| n == name);
        let line: Vec<u8> = if let Some((_, body)) = alias {
            [&name[..], b" is an alias for ", &body[..]].concat()
        } else if NAMES.contains(name) {
            [&name[..], b" is built into the shell"].concat()
        } else if CONTROL.contains(name) {
            [&name[..], b" is a control word"].concat()
        } else if FILTERS.contains(name) {
            [&name[..], b" is a filter, run after |"].concat()
        } else {
            found = false;
            [&name[..], b" not found"].concat()
        };
        out.writeln(&line);
    }
    found
}

impl Session {
    /// Set alias `name` to `body`; false, with nothing changed, when the body
    /// passes `MAX_VALUE` or a new name would pass `MAX_ALIASES`.
    pub fn set_alias(&mut self, name: &[u8], body: &[u8]) -> bool {
        if body.len() > MAX_VALUE {
            return false;
        }
        let full = self.aliases.len() >= MAX_ALIASES;
        match self.aliases.iter_mut().find(|(n, _)| n == name) {
            Some((_, v)) => *v = body.to_vec(),
            None if full => return false,
            None => self.aliases.push((name.to_vec(), body.to_vec())),
        }
        true
    }
}
