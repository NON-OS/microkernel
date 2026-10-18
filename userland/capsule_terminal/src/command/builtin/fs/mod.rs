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

//! The filesystem command set: the shell verbs a user expects (pwd, cd, ls,
//! cat, mkdir, touch, rm, mv, cp, stat). Each speaks to the vfs capsule over
//! IPC with the terminal's own pid, so the server's anti-impersonation check
//! passes, and resolves paths against the current working directory.

mod caller;
mod cat;
mod cd;
mod cp;
mod filter_file;
mod find;
mod glob;
mod grep;
mod grep_match;
mod grep_paint;
mod grep_scan;
mod head;
mod ls;
mod ls_date;
mod ls_dots;
mod ls_emit;
mod ls_flags;
mod ls_list;
mod ls_long;
mod ls_meta;
mod ls_num;
mod ls_total;
mod mkdir;
mod mv;
mod pwd;
mod read_file;
mod rm;
mod rmdir;
mod stat;
mod tail;
mod touch;
mod tree;
pub mod tree_render;
mod wc;

pub use self::{
    cat::cat, cd::cd, cp::cp, filter_file::filter_file, find::find, grep::grep, head::head, ls::ls,
    mkdir::mkdir, mv::mv, pwd::pwd, rm::rm, rmdir::rmdir, stat::stat, tail::tail, touch::touch,
    tree::tree, wc::wc,
};
pub(super) use caller::{abspath, pid};
