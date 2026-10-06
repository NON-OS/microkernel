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


//! The tools built in this tree that an image does not carry in its store:
//! the packages of userland/linux_userland/Userland.mk. The market lists each
//! as `linux.nonos-<tool>`, pinned by the BLAKE3 of its content (its program
//! and the data it reads, as one tar), and a NONOS package mirror serves that
//! content at a path named by the pin. A mirror can therefore serve nothing
//! but the bytes the signed index already pins, and any mirror will do. Pure,
//! so capsule_linux_proofs holds the names and paths.

use alloc::string::String;

/// The listing namespace in-tree tools sit under, inside Alpine's family:
/// the kernel passes `nonos-rg` for `linux.nonos-rg`, as it passes a Qwen
/// tier's name.
pub const PREFIX: &str = "nonos-";

/// The tool `pkg` names, or None when it is not in the namespace or names
/// nothing a tool could be called.
pub fn tool(pkg: &str) -> Option<&str> {
    let name = pkg.strip_prefix(PREFIX)?;
    let fits = |c: u8| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-' || c == b'.';
    let ok = !name.is_empty() && !name.starts_with(['-', '.']) && name.bytes().all(fits);
    ok.then_some(name)
}

/// Where a mirror serves the content pinned as `pin`.
pub fn content_path(pin: &[u8; 32]) -> String {
    let mut path = String::from("/linux/");
    for b in pin {
        path.push_str(&alloc::format!("{b:02x}"));
    }
    path.push_str(".tar");
    path
}
