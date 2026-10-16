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
//! Usage of the commands that walk and change the file tree.

pub(crate) const FILES: &[(&[u8], &[u8], &[u8])] = &[
    (
        b"ls",
        b"ls [-l -a -h -1 -R -t -S] [path...]",
        b"list a directory; -l long, -a hidden, -h human sizes, -R recurse, -t by time, -S by size",
    ),
    (b"tree", b"tree [path]", b"draw the shape of a directory and everything under it"),
    (b"cat", b"cat [-n] <file...>", b"print files; -n numbers the lines"),
    (b"cd", b"cd [path]", b"change directory; no argument goes to $HOME, /home/nonos"),
    (b"pwd", b"pwd", b"print the working directory"),
    (b"mkdir", b"mkdir [-p] <dir...>", b"make directories; -p makes parents too"),
    (b"touch", b"touch <path...>", b"create empty files; a file that exists is left as it is"),
    (
        b"rm",
        b"rm [-r -f] <path...>",
        b"remove; -r recurses into directories, -f ignores what is missing",
    ),
    (b"rmdir", b"rmdir <dir...>", b"remove empty directories"),
    (b"mv", b"mv <src> <dst>", b"move or rename"),
    (b"cp", b"cp [-r] <src> <dst>", b"copy; -r recurses into directories"),
    (b"stat", b"stat <path>", b"kind, size, write bit and modification time of one path"),
    (b"find", b"find [path] [-name <pat>] [-type f|d]", b"walk a tree, filtering by name or kind"),
    (b"du", b"du [path]", b"how many bytes the files under a path hold"),
];
