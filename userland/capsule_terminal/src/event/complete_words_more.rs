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

//! The command names for the system, text, and the late additions.

/// The clock, the environment, processes and the network.
pub(super) const SYSTEM: &[&[u8]] =
    &[b"date", b"env", b"ps", b"ifconfig", b"ip", b"nslookup", b"nym", b"id", b"sys", b"apps"];

/// Filtering text, running programs, and the shell's own settings.
pub(super) const TEXT: &[&[u8]] = &[
    b"grep", b"wc", b"head", b"tail", b"cut", b"run", b"open", b"set", b"unset", b"alias",
    b"unalias", b"sort", b"uniq", b"nl", b"install", b"type", b"which", b"tree",
];

// Reachable at the prompt and, until this line, never offered by Tab.
pub(super) const LATE: &[&[u8]] = &[
    b"jobs",
    b"fg",
    b"bg",
    b"rmdir",
    b"neofetch",
    b"commands",
    b"profile",
    b"theme",
    b"tac",
    b"rev",
    b"qwen",
];
