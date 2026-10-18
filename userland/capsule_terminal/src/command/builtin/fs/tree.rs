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
use crate::command::output::Output;
use crate::term::cwd::resolve;
use crate::term::state::State;

#[path = "tree_show.rs"]
mod tree_show;
#[path = "tree_summary.rs"]
mod tree_summary;

use tree_show::show;

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
    show(&mut out, rel);
    state.last_status = 0;
}
