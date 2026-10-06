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
//! Putting a package's files into the store, under the Linux root.

//! What an unpack did, as numbers in the log: files, links, and anything the
//! archive held that had nowhere to go. A dropped entry is never silent.

use alloc::format;

use nonos_libc::mk_debug;

pub(super) fn say(files: usize, links: usize, linked: Option<usize>, dropped: u32) {
    let written = match linked {
        Some(n) => format!("{n}"),
        None => "0 (table not written)".into(),
    };
    let line = format!(
        "[LINUX] unpacked files={files} links={links} linked={written} dropped={dropped}\n"
    );
    let _ = mk_debug(line.as_ptr(), line.len());
}
