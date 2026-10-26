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

//! A small package's sources, shared by the pack's tests.

use nonos_linux_pack::{Kind, Source};

pub fn src<'a>(name: &str, kind: Kind, exec: bool, bytes: &'a [u8]) -> Source<'a> {
    Source { name: name.as_bytes().to_vec(), kind, exec, bytes }
}

/// vim's shape in little: a program, a link to it, a runtime directory.
pub fn vimish() -> Vec<Source<'static>> {
    vec![
        src("usr/bin/vim", Kind::File, true, &[0x7f, b'E', b'L', b'F', 1, 2, 3]),
        src("usr/bin/vi", Kind::Link, false, b"vim"),
        src("usr/share/vim", Kind::Dir, false, b""),
        src("usr/share/vim/vim91", Kind::Dir, false, b""),
        src("usr/share/vim/vim91/defaults.vim", Kind::File, false, b"set nocompatible\n"),
        src("usr/share/vim/vim91/empty.txt", Kind::File, false, b""),
        src("usr/share/vimrc", Kind::File, false, &[b'x'; 5000]),
    ]
}
