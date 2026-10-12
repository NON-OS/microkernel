// NONOS Operating System (AGPL-3.0-or-later)
//! Every entry point over one blob, results ignored: what matters to the
//! mutation sweep is that none of them panics or fails to return.

use crate::arch::fdt::find::{
    cpus, framebuffer, gic, memory, msi, pci, psci, rtc, smmu, timer, uart,
};
use crate::arch::fdt::irq::{gic_edge, gic_intid};
use crate::arch::fdt::Fdt;

pub fn all(bytes: &[u8]) {
    let Ok(fdt) = Fdt::from_bytes(bytes) else { return };
    let mut ranges = [crate::arch::fdt::find::memory::MemoryRange { base: 0, size: 0 }; 8];
    let mut ids = [0u64; 8];
    let mut held = [(0u64, 0u64); 8];
    let _ = memory::find(&fdt, &mut ranges);
    let _ = cpus::find(&fdt, &mut ids);
    let _ = (uart::find(&fdt), gic::find(&fdt), timer::find(&fdt));
    let _ = (pci::find(&fdt), rtc::find(&fdt), fdt.stdout(), fdt.initrd());
    let _ = (smmu::find(&fdt), smmu::pci_iommu_map(&fdt), pci::interrupt_map(&fdt));
    let _ = (msi::pci_msi_map(&fdt), psci::find(&fdt), framebuffer::find(&fdt));
    let _ = fdt.reserved_memory(&mut held);
    let _ = fdt.reservations().count();
    let _ = fdt.resolve(b"serial0/bluetooth");
    let _ = fdt.chosen_bytes(b"rng-seed");
    for node in fdt.nodes().take(256).flatten() {
        let _ = fdt.own_cells(&node);
        let _ = fdt.is_enabled(&node);
        let _ = fdt.mmio(&node, 0);
        let _ = fdt.interrupt_parent(&node);
        if let Ok(Some(spec)) = fdt.interrupt(&node, 0) {
            let _ = (gic_intid(spec.cells), gic_edge(spec.cells));
        }
        if let Ok(children) = fdt.children(&node) {
            let _ = children.count();
        }
    }
}
