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

//! The deeper help pages: keys and shell syntax.

pub const KEYS: [(&[u8], &[u8]); 8] = [
    (b"move", b"Ctrl-A start   Ctrl-E end   Ctrl-Left/Right by word"),
    (b"cut", b"Ctrl-W word   Ctrl-K to end   Ctrl-U line   Ctrl-Y put back   Ctrl-D delete"),
    (b"complete", b"Tab completes a command or a path"),
    (b"recall", b"Up/Down walk history   Ctrl-R search it   Ctrl-C abandon the line"),
    (b"history", b"!! the last command   !n the nth   !text the last starting with text"),
    (b"tabs", b"Ctrl+Shift+T new   Ctrl+Shift+W close   Ctrl+PgUp/PgDn switch"),
    (b"view", b"Ctrl-B side rail   Ctrl+= / Ctrl+- font size   Ctrl-L clear"),
    (b"theme", b"theme or profile to change colours"),
];

pub const SHELL: [(&[u8], &[u8]); 6] = [
    (b"pipe", b"a | b          feed a's output to b"),
    (b"redirect", b"a > f   a >> f   a < f   to, onto, or from a file"),
    (b"chain", b"a && b   a || b   a ; b   on success, on failure, always"),
    (b"jobs", b"a &   jobs   fg   bg      run in the background and bring it back"),
    (b"alias", b"alias ll ls -l   unalias ll   set   unset   env"),
    (b"run", b"run or open <app>   exec   type or which <name>   exit"),
];
