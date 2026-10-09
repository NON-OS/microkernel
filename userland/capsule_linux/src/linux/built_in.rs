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

//! The program a machine runs when its store names none.

use alloc::vec::Vec;

use super::launch::Launch;
use super::origin::Origin;

/// The built-in program: BusyBox 1.36.1, built static against musl from the
/// pinned upstream release by `tools/nonos-busybox-build`, embedded so a
/// machine with nothing in the store still runs a real Linux binary.
static BUILT_IN: &[u8] = include_bytes!("../../guests/busybox.elf");

/// Where BusyBox puts the links to its programs.
const BIN_DIRS: [&[u8]; 4] = [b"/bin", b"/sbin", b"/usr/bin", b"/usr/sbin"];

/// The name the built-in BusyBox runs `path` under when the Linux tree has
/// no such file: `busybox` itself, or one of its programs named bare or in a
/// bin directory, as BusyBox links them. `None` for any other path. A file
/// the store does hold always runs instead, proved like any other.
pub(crate) fn serves(path: &[u8]) -> Option<&[u8]> {
    let name = match path.iter().rposition(|&b| b == b'/') {
        None => path,
        Some(at) if BIN_DIRS.contains(&&path[..at]) => &path[at + 1..],
        Some(_) => return None,
    };
    (name == b"busybox" || super::applets::has(BUILT_IN, name)).then_some(name)
}

/// The built-in's size, which each of its programs shows as a file's.
pub(crate) fn size() -> u64 {
    BUILT_IN.len() as u64
}

pub(super) fn built_in() -> Launch {
    Launch {
        path: b"/bin/busybox".to_vec(),
        bytes: BUILT_IN.to_vec(),
        origin: Origin::BuiltIn,
        args: Vec::new(),
        argv0: None,
    }
}
