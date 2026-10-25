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
//! The nox names that work on files and paths.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Files {
    Where,
    In,
    Ls,
    Read,
    Pull,
    Push,
    Write,
    Copy,
    Mk,
    Rm,
    Keep,
    Mv,
    Stat,
    Find,
    Du,
    Touch,
    Basename,
    Dirname,
}

pub(crate) const NAMES: &[(&[u8], Files)] = &[
    (b"where", Files::Where),
    (b"pwd", Files::Where),
    (b"in", Files::In),
    (b"cd", Files::In),
    (b"ls", Files::Ls),
    (b"dir", Files::Ls),
    (b"read", Files::Read),
    (b"cat", Files::Read),
    (b"pull", Files::Pull),
    (b"push", Files::Push),
    (b"write", Files::Write),
    (b"copy", Files::Copy),
    (b"cp", Files::Copy),
    (b"mk", Files::Mk),
    (b"mkdir", Files::Mk),
    (b"rm", Files::Rm),
    (b"del", Files::Rm),
    (b"keep", Files::Keep),
    (b"mv", Files::Mv),
    (b"move", Files::Mv),
    (b"stat", Files::Stat),
    (b"find", Files::Find),
    (b"du", Files::Du),
    (b"touch", Files::Touch),
    (b"basename", Files::Basename),
    (b"dirname", Files::Dirname),
];
