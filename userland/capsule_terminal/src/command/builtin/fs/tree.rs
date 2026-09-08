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

//! `tree`: the shape of a directory, drawn.
//!
//! `ls` says what is in one place and `find` says everything under it as a
//! flat list. Neither shows how a tree is arranged, which is the thing anyone
//! opening an unfamiliar directory wants first. One vfs call returns every
//! path beneath the prefix, so this costs the same as a recursive `find`.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;

use super::pid;
use super::tree_render::{counts, render};
use crate::command::output::Output;
use crate::term::cwd::resolve;
use crate::term::state::State;

/// Most nodes drawn before stopping.
///
/// Deliberately well under `SCROLLBACK_ROWS`, because the scrollback is a ring
/// and a tree allowed to fill it would push out everything the reader ran
/// before, including the command they are reading the tree to answer.
const MAX_NODES: usize = 180;

pub fn tree(state: &mut State, argv: &[&[u8]]) {
    let arg = argv.get(1).copied().unwrap_or(b".");
    let mut base = resolve(state.cwd.as_bytes(), arg);
    if !base.ends_with(b"/") {
        base.push(b'/');
    }

    let owner = pid(state);
    let paths = match vfs::list_paths(owner, &base) {
        Ok(p) => p,
        Err(e) => {
            Output::new(&mut state.scrollback).writeln(e.as_bytes());
            state.last_status = 1;
            return;
        }
    };

    // The vfs answers with absolute paths; the drawing is relative to the root
    // being shown, which is printed once as its own line.
    let rel: Vec<Vec<u8>> = paths
        .iter()
        .filter_map(|p| p.as_bytes().strip_prefix(base.as_slice()))
        .filter(|r| !r.is_empty())
        .map(|r| r.to_vec())
        .collect();

    let mut out = Output::new(&mut state.scrollback);
    out.writeln(&base);
    if rel.is_empty() {
        out.writeln(b"  (empty)");
        state.last_status = 0;
        return;
    }

    let lines = render(&rel);
    let shown = lines.len().min(MAX_NODES);
    for row in lines.iter().take(shown) {
        // Directories take the same colour they take in a listing, so the two
        // commands agree about what a directory looks like.
        if row.is_dir {
            out.writeln_dir(&row.line[..row.name_at], &row.line[row.name_at..]);
        } else {
            out.writeln(&row.line);
        }
    }
    if lines.len() > shown {
        out.writeln(b"... more, narrow the path to see the rest");
    }

    let (dirs, files) = counts(&lines);
    out.writeln(&summary(dirs, files));
    state.last_status = 0;
}

/// "N directories, M files", built without a formatter.
fn summary(dirs: usize, files: usize) -> Vec<u8> {
    let mut line = Vec::new();
    push_num(&mut line, dirs);
    line.extend_from_slice(if dirs == 1 { b" directory, " } else { b" directories, " });
    push_num(&mut line, files);
    line.extend_from_slice(if files == 1 { b" file" } else { b" files" });
    line
}

fn push_num(out: &mut Vec<u8>, mut n: usize) {
    if n == 0 {
        out.push(b'0');
        return;
    }
    let mut digits = [0u8; 20];
    let mut i = digits.len();
    while n > 0 {
        i -= 1;
        digits[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    out.extend_from_slice(&digits[i..]);
}
