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

//! MkAcpiFirstPass: the first publish pass is done (L55). Only this
//! capsule's thread group may say so; saying it again changes nothing.

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const FIRST_PASS: i64 = tag4(b"MAFP");

pub fn first_pass_done() -> Result<(), Errno> {
    ok(mk_syscall_raw(FIRST_PASS, [0; 6])).map(|_| ())
}
