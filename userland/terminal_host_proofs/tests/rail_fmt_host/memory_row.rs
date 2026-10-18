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

//! The memory row.

use crate::paint::rail_fmt::mib_into;
use crate::paint::rail_metric::pair;
use crate::rail::mem::summarize;
use crate::rail::metrics::Proc;

#[test]
fn a_memory_row_shows_the_kernel_total_beside_the_resident_sum() {
    let mut b = [0u8; 48];
    let m = summarize(&[Proc { pid: 1, mem_kb: 3952, ..Proc::EMPTY }], 8192);
    assert_eq!(pair(&mut b, m.used_kb, m.total_kb, mib_into), "3.8 MB / 8.0 MB");
}
