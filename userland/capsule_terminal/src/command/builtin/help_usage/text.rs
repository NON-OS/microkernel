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
//! Usage of the commands that read, search and filter lines of text.

pub(crate) const TEXT: &[(&[u8], &[u8], &[u8])] = &[
    (
        b"head",
        b"head [-n <count>] <file...>",
        b"first lines of each file, ten by default; after a pipe, of the piped lines",
    ),
    (
        b"tail",
        b"tail [-n <count>] <file...>",
        b"last lines of each file, ten by default; after a pipe, of the piped lines",
    ),
    (
        b"grep",
        b"grep [-c -i -n -r -v] <pattern> <path...>",
        b"search; -i ignores case, -n numbers, -r recurses, -v inverts, -c counts; no -r in a pipe",
    ),
    (
        b"wc",
        b"wc [-l -w -c] <file...>",
        b"count lines, words, bytes, with a total for several files; after a pipe, the piped lines",
    ),
    (b"echo", b"echo [text...]", b"write the arguments back"),
    (b"sort", b"sort [-n -r -u] [file...]", b"sort lines; -n numeric, -r reverse, -u unique"),
    (b"uniq", b"uniq [-c] [file...]", b"collapse repeated neighbouring lines; -c counts each run"),
    (
        b"cut",
        b"cut [-d <char>] [-f <n>] [file...]",
        b"the n-th field of each line split on the character; a space and field 1 by default",
    ),
    (b"nl", b"nl [file...]", b"number the lines"),
    (b"tac", b"tac [file...]", b"reverse the order of the lines"),
    (b"rev", b"rev [file...]", b"reverse the characters within each line"),
];
