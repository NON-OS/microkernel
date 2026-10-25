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
//! Usage of the commands that install and run apps, and leave the terminal.

pub(crate) const APPS: &[(&[u8], &[u8], &[u8])] = &[
    (b"market", b"market", b"the packages the market service offers"),
    (
        b"install",
        b"install <name> [argv...]",
        b"verify, load and start the capsule held at /capsules/<name>.* in the store",
    ),
    (
        b"pkg",
        b"pkg install <path> [--yes] | remove <name> | status",
        b"install a package, remove one, or list what is installed",
    ),
    (
        b"git",
        b"git init | clone <url> | add | status | commit -m <msg> | log | push | remote",
        b"a git client for repositories in the file store, over the chosen network",
    ),
    (b"nox", b"nox [command]", b"the nox command index, or a command by its nox name"),
    (
        b"run",
        b"run <app>",
        b"bring an app's window forward: files editor settings calc about procs term",
    ),
    (b"open", b"open <app>", b"the same as run"),
    (
        b"exec",
        b"exec <name> [argv...]",
        b"load a store capsule as this terminal's child and run it in the foreground",
    ),
    (b"exit", b"exit", b"close this terminal; quit is the same"),
    (b"qwen", b"qwen [tier] [question]", b"chat with a Qwen model on this machine, offline"),
];
