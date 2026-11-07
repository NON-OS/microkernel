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

//! Any bytes as a flattened device tree, through the kernel's own parser and
//! every walker and finder the boot path calls. None panics or loops (a hang
//! is a libFuzzer timeout). An accepted blob is no larger than the input and
//! its memory reservation, structure and strings blocks lie inside it; every
//! node's body and every property's name and value lie inside the blocks
//! they were read from; no node is deeper than the depth limit; a finder
//! that fills a caller's array never reports more entries than it holds.

#![no_main]

use fdt_proofs::arch::fdt::find::{
    cpus, framebuffer, gic, memory, msi, pci, psci, rtc, smmu, timer, uart,
};
use fdt_proofs::arch::fdt::irq::{gic_edge, gic_intid};
use fdt_proofs::arch::fdt::limits::{MAX_BLOB, MAX_DEPTH};
use fdt_proofs::arch::fdt::{Fdt, NodeRef};
use libfuzzer_sys::fuzz_target;

fn inside(outer: &[u8], inner: &[u8]) -> bool {
    let (o, i) = (outer.as_ptr() as usize, inner.as_ptr() as usize);
    inner.is_empty() || (i >= o && i + inner.len() <= o + outer.len())
}

fn blocks(fdt: &Fdt, data: &[u8]) {
    assert!(fdt.blob.len() <= data.len() && fdt.blob.len() <= MAX_BLOB);
    assert!(inside(data, fdt.blob));
    for block in [fdt.rsvmap, fdt.structure, fdt.strings] {
        assert!(inside(fdt.blob, block), "a block outside the blob");
    }
}

fn node(fdt: &Fdt, n: &NodeRef) {
    assert!(n.body <= fdt.structure.len(), "a node body at {} past the structure", n.body);
    assert!(n.depth <= MAX_DEPTH, "a node {} deep", n.depth);
    assert!(inside(fdt.structure, n.name));
    for prop in fdt.properties(n).take(256).flatten() {
        assert!(inside(fdt.structure, prop.data), "a property value outside the structure");
        assert!(inside(fdt.strings, prop.name), "a property name outside the strings");
    }
    let _ = (fdt.own_cells(n), fdt.is_enabled(n), fdt.mmio(n, 0), fdt.interrupt_parent(n));
    let _ = (fdt.dma_window(n), fdt.chain(n).map(|c| c.as_slice().len()));
    if let Ok(Some(spec)) = fdt.interrupt(n, 0) {
        let _ = (gic_intid(spec.cells), gic_edge(spec.cells));
    }
    if let Ok(children) = fdt.children(n) {
        for child in children.take(256).flatten() {
            assert_eq!(child.depth, n.depth + 1);
            assert!(child.body <= fdt.structure.len());
        }
    }
}

fn finders(fdt: &Fdt) {
    let mut ranges = [memory::MemoryRange { base: 0, size: 0 }; 8];
    if let Ok(n) = memory::find(fdt, &mut ranges) {
        assert!(n <= ranges.len());
    }
    let mut ids = [0u64; 8];
    if let Ok(n) = cpus::find(fdt, &mut ids) {
        assert!(n <= ids.len());
    }
    let mut held = [(0u64, 0u64); 8];
    if let Ok(n) = fdt.reserved_memory(&mut held) {
        assert!(n <= held.len());
    }
    let _ = (uart::find(fdt), gic::find(fdt), timer::find(fdt), rtc::find(fdt));
    let _ = (pci::find(fdt), pci::interrupt_map(fdt), psci::find(fdt));
    let _ = (smmu::find(fdt), smmu::pci_iommu_map(fdt), msi::pci_msi_map(fdt));
    let _ = framebuffer::find(fdt);
    let _ = (fdt.stdout(), fdt.initrd(), fdt.acpi_rsdp(), fdt.tables());
    let _ = fdt.reservations().take(256).count();
    let _ = (fdt.resolve(b"serial0/bluetooth"), fdt.find_path(b"/chosen"));
    if let Ok(Some(seed)) = fdt.chosen_bytes(b"rng-seed") {
        assert!(inside(fdt.structure, seed));
    }
}

fuzz_target!(|data: &[u8]| {
    let Ok(fdt) = Fdt::from_bytes(data) else { return };
    blocks(&fdt, data);
    for n in fdt.nodes().take(256).flatten() {
        node(&fdt, &n);
    }
    finders(&fdt);
});
