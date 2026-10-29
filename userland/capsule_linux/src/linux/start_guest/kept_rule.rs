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

//! Whether an identity kept by run_net::early is for the program a start is
//! given: the same path, and the very bytes, by their place and length, which
//! the launch holds unchanged from the one to the other. A copy of the same
//! bytes elsewhere is not them, and is proved again. Pure.

/// The path and the bytes' place and length an identity was proved for.
pub type Proved<'a> = (&'a [u8], usize, usize);

pub fn same(kept: Proved<'_>, path: &[u8], bytes: &[u8]) -> bool {
    kept == (path, bytes.as_ptr() as usize, bytes.len())
}
