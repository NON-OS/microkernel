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


//! A grant's teardown and a held run's free, pinned whole (whole): a kept
//! run is entered before the registry is asked and is not counted released
//! (L78, L80); an unconfined one not kept is counted released once, at its
//! teardown; a run the registry let go is freed unless its wipe still keeps
//! it, and counted released then only if it was kept (glue_tests and
//! kept_count_tests read the same orders and notes). Its pages come out of
//! the holder's own tables first, from any context. A VMBus channel's
//! page the host may still hold a GPADL over is kept as a wiped function's
//! is, until the host says it tore the last one down.

use super::code::flat;
use super::whole::whole;

const TEARDOWN: &str = include_str!("../../../../../src/hardware/broker/dma/teardown.rs");
const HELD: &str = include_str!("../../../../../src/hardware/broker/dma/held_run.rs");

const TEARS: [&str; 18] = [
    "pub(super) fn teardown(g: &DmaGrant, unmap_pages: bool, released: bool) {",
    "if super::super::holder_space::unmap_from_holder(g.pid, g.user_va, g.length) {",
    "super::va::release(g.pid, g.user_va, g.length); } else if unmap_pages {",
    "let _ = crate::memory::paging::unmap_user_dma(VirtAddr::new(g.user_va), g.length as usize); }",
    "if !super::super::confine::unmap(g.pid, g.device_addr, g.length, g.confined) {",
    r#"crate::sys::serial::println( b"[DMA] grant still reachable by its device; frames"#,
    r#"quarantined", ); return; }"#,
    "let pci = crate::hardware::broker::pci_index::lookup(g.device_id).is_some();",
    "let host = crate::hardware::broker::vmbus::host_may_reach(g.device_id);",
    "if super::kept_rule::keeps(pci, g.confined, released) || host {",
    "super::scrub::scrub(g.physical_start, g.length);",
    "super::kept::keep(g.device_id, g.claim_epoch, g.physical_start, g.length);",
    "let run = (g.physical_start, g.length);",
    "if !crate::kernel_core::surface_registry::pin::grant_run::hold(run.0, run.1) {",
    "let _ = super::kept::defer(run.0, run.1); } return; }",
    "if !g.confined { crate::memory::iommu::note_unconfined_released(1); }",
    "if crate::kernel_core::surface_registry::pin::grant_run::hold(g.physical_start, g.length) {",
    "return; } super::held_run::free_held_run(g.physical_start, g.length); }",
];

const FREES: [&str; 4] = [
    "pub fn free_held_run(physical_start: u64, length: u64) {",
    "let kept = super::kept::defer(physical_start, length); if kept == Some(true) { return; }",
    "if kept == Some(false) { crate::memory::iommu::note_unconfined_released(1); }",
    "free_now(physical_start, length); }",
];

#[test]
fn a_teardown_keeps_or_counts_and_a_let_go_frees_as_pinned() {
    whole(&flat(TEARDOWN), "teardown", &TEARS);
    whole(&flat(HELD), "free_held_run", &FREES);
}
