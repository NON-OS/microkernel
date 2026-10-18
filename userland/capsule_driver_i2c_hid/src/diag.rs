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

//! The driver's lines on the boot console: what it bound, gave up or left,
//! in plain words. No report, frame or device identifier is ever printed.

pub fn line(s: alloc::string::String) {
    let b = s.as_bytes();
    let len = b.len().min(256);
    let _ = nonos_libc::mk_debug(b.as_ptr(), len);
}
