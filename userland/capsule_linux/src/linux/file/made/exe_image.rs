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

/*
 * What exec tells /proc about the image it built: the file, its name, and
 * where its arguments and environment lie.
 */

use alloc::vec::Vec;

use super::exe::{comm_of, record, Exe};

/*
 * argv's strings run up from the first, then the environment's, then
 * `top`, as the stack builder placed them at `at`.
 */
pub fn record_image(pid: u32, argv: &[Vec<u8>], at: &[u64], top: u64) {
    let (Some(first), Some(name)) = (at.first(), argv.first()) else {
        return;
    };
    let env = at.get(argv.len()).copied().unwrap_or(top);
    /* A name with no directory is not yet the file it names; exec says which. */
    let path = if name.first() == Some(&b'/') { name.clone() } else { Vec::new() };
    let start_ms = crate::linux::call::family_ms();
    record(pid, Exe { path, comm: comm_of(name), args: *first, env, end: top, start_ms });
}
