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

//! Which check a fetched package failed, told to init for the person who
//! asked (refuse::tell), and the debug line get.rs says.

use nonos_libc::mk_debug;

use super::super::index::Pkg;
use super::super::refuse::tell;

/// The check `get::fetch` stopped at.
pub(super) enum Check {
    /// The market's pin on the package the person chose.
    Market,
    /// The signed index's checksum, and the data hash its control records.
    Index,
    /// Unpacking it ran out of memory.
    Memory,
}

/// `got`: whether any bytes arrived, so a download that never came is not
/// told as a package that did not match.
pub(super) fn told(pkg: &Pkg, got: bool, check: Check) {
    let what = match (got, check, pkg.checksum.is_some()) {
        (false, Check::Market | Check::Index, _) => "it did not download",
        (_, Check::Market, _) => "it is not the one the market listed",
        (_, Check::Index, false) => "the signed index gives it no checksum",
        (_, Check::Index, true) => "its checksum did not match the signed index",
        (_, Check::Memory, _) => "not enough memory to unpack it",
    };
    tell(&pkg.name, &pkg.version, what);
}

pub(super) fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
