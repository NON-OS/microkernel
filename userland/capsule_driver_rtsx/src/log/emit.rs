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

//! Sending a line to the kernel console, and the two shapes most lines
//! take: a step that stopped, and a step's result passed on.

use nonos_libc::mk_debug;

use super::line::Line;
use crate::error::{Result, RtsxError};

pub fn emit(line: &mut Line) {
    let bytes = line.finish();
    let _ = mk_debug(bytes.as_ptr(), bytes.len());
}

/// "rtsx: <what>: <error name>", for a step that stopped.
pub fn failed(what: &[u8], err: RtsxError) {
    emit(Line::start().text(what).text(b": ").text(err.name()));
}

/// A step's result, with its failure named in the log on the way out.
pub fn step<T>(what: &[u8], r: Result<T>) -> Result<T> {
    if let Err(e) = &r {
        failed(what, *e);
    }
    r
}
