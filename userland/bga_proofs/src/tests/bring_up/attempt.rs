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

//! One bring-up attempt as the shipping driver makes it, its primaries on
//! the test heap instead of the kernel's pool.

use crate::discover::Found;
use crate::error::BgaResult;
use crate::setup::Driver;
use crate::tests::wire_sweep::heap::Heap;

/// One bring-up attempt on `f`, its primaries on the test heap.
pub fn run(f: Found) -> BgaResult<Driver<Heap>> {
    crate::setup::run(f, Heap::new(1 << 16))
}
