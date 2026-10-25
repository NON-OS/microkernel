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
//! Usage of the commands that set the shell's variables, aliases and look.

pub(crate) const SETTINGS: &[(&[u8], &[u8], &[u8])] = &[
    (b"env", b"env", b"the shell variables that are set"),
    (b"set", b"set [name value]", b"list the shell variables, or set one; use it as $name"),
    (b"unset", b"unset <name>", b"remove a shell variable"),
    (b"alias", b"alias [name expansion]", b"list the aliases, or define one"),
    (b"unalias", b"unalias <name>", b"remove an alias"),
    (b"theme", b"theme [name]", b"switch the terminal profile, or list them"),
    (b"clear", b"clear", b"empty the scrollback; Ctrl-L does the same"),
    (b"help", b"help [command]", b"the grouped list, or one command in detail"),
];
