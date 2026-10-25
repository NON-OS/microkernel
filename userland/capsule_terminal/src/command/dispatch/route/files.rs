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
//! The shell names that work on files; the line filters share one command.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Files {
    Pwd,
    Cd,
    Ls,
    Cat,
    Mkdir,
    Touch,
    Rm,
    Mv,
    Cp,
    Stat,
    Rmdir,
    Find,
    Tree,
    Head,
    Tail,
    Grep,
    Wc,
    Filter,
}

pub(crate) const NAMES: &[(&[u8], Files)] = &[
    (b"pwd", Files::Pwd),
    (b"cd", Files::Cd),
    (b"ls", Files::Ls),
    (b"dir", Files::Ls),
    (b"cat", Files::Cat),
    (b"mkdir", Files::Mkdir),
    (b"touch", Files::Touch),
    (b"rm", Files::Rm),
    (b"del", Files::Rm),
    (b"mv", Files::Mv),
    (b"cp", Files::Cp),
    (b"stat", Files::Stat),
    (b"rmdir", Files::Rmdir),
    (b"find", Files::Find),
    (b"tree", Files::Tree),
    (b"head", Files::Head),
    (b"tail", Files::Tail),
    (b"grep", Files::Grep),
    (b"wc", Files::Wc),
    (b"sort", Files::Filter),
    (b"uniq", Files::Filter),
    (b"cut", Files::Filter),
    (b"nl", Files::Filter),
    (b"tac", Files::Filter),
    (b"rev", Files::Filter),
];
