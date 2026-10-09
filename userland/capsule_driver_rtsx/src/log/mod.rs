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

//! The driver's lines on the kernel console, which `log rtsx` shows on a
//! machine with no serial port. Printed only when the build grants the
//! Debug capability; otherwise the call is refused and the driver goes on.

mod line;

use nonos_libc::mk_debug;

pub use line::Line;

pub fn emit(line: &mut Line) {
    let bytes = line.finish();
    let _ = mk_debug(bytes.as_ptr(), bytes.len());
}

/// "rtsx: <what>: <error name>", for a step that stopped.
pub fn failed(what: &[u8], err: crate::error::RtsxError) {
    emit(Line::start().text(what).text(b": ").text(err.name()));
}

/// A step's result, with its failure named in the log on the way out.
pub fn step<T>(what: &[u8], r: crate::error::Result<T>) -> crate::error::Result<T> {
    if let Err(e) = &r {
        failed(what, *e);
    }
    r
}
