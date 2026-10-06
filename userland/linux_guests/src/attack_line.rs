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

//! The attack suite's line for a guest's attempt:
//! `[ATTACK] <name> refused: <how>` or `[ATTACK] <name> ESCAPED: <what>`,
//! beside the guest's own line, for the guests that are one of its attacks.

use crate::sys::out;

/// The attack a guest's attempts belong to, if any.
pub fn attack_of(guest: &str) -> Option<&'static str> {
    match guest {
        // `..` past the root, a symlink out, /proc of other pids.
        "fs" | "proc" => Some("linux-fs-escape"),
        // The parent's pages after fork, a sibling's, the kernel half.
        "separation" | "reader" | "bounds" => Some("linux-foreign-memory"),
        // A native NØNOS syscall number from a Linux process.
        "native" => Some("linux-raw-syscall"),
        _ => None,
    }
}

pub fn refused(attack: &str, guest: &str, what: &str, errno: i64) {
    out(format!("[ATTACK] {attack} refused: {guest} {what}, errno {errno}\n").as_bytes());
}

pub fn escaped(attack: &str, guest: &str, what: &str, how: &str) {
    out(format!("[ATTACK] {attack} ESCAPED: {guest} {what}: {how}\n").as_bytes());
}
