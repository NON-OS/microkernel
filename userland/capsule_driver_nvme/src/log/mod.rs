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

//! Bring-up lines on the serial console. A laptop's internal SSD is debugged
//! from these alone: which controller was taken, which admin command failed
//! with which status, and why a namespace got no I/O queue. The kernel prints
//! them only when the build grants the Debug capability
//! (`capsule-serial-debug`); without it the call is refused and the driver
//! carries on.

mod line;

use nonos_libc::mk_debug;

pub use line::Line;

pub fn emit(line: &mut Line) {
    let bytes = line.finish();
    let _ = mk_debug(bytes.as_ptr(), bytes.len());
}
