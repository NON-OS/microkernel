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

//! Memory summary figures from the process table.

use crate::rail::mem::summarize;
use crate::rail::metrics::Proc;
use crate::rail::value::Metric;

fn proc_with(pid: u32, mem_kb: u64) -> Proc {
    Proc { pid, mem_kb, ..Proc::EMPTY }
}

#[test]
fn memory_used_is_the_resident_sum_and_saturates() {
    let live = [proc_with(1, 4096), proc_with(2, 512), proc_with(3, 0)];
    assert_eq!(summarize(&live, 0).used_kb, Metric::Known(4608));
    let huge = [proc_with(1, u64::MAX), proc_with(2, 1024)];
    assert_eq!(summarize(&huge, 0).used_kb, Metric::Known(u64::MAX));
}

#[test]
fn an_empty_process_table_is_unknown_memory_rather_than_zero() {
    let m = summarize(&[], 0);
    assert_eq!(m.used_kb, Metric::Unknown);
}

#[test]
fn only_swap_has_no_source_at_all() {
    let m = summarize(&[proc_with(1, 8)], 2 * 1024 * 1024);
    assert_eq!(m.total_kb, Metric::Known(2 * 1024 * 1024));
    assert!(m.swap_used_kb == Metric::Unsupported);
}

#[test]
fn an_unreadable_memory_map_leaves_the_total_unmeasured_rather_than_absent() {
    let m = summarize(&[proc_with(1, 8)], 0);
    assert_eq!(m.total_kb, Metric::Unknown);
    assert!(m.total_kb != Metric::Unsupported);
}
