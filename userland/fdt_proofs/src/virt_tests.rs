// NONOS Operating System (AGPL-3.0-or-later)
//! QEMU `virt` with a GICv3: what the boot path reads from it.

use crate::arch::fdt::find::memory::MemoryRange;
use crate::arch::fdt::find::{cpus, gic, memory, pci, rtc, timer, uart};
use crate::arch::fdt::Fdt;
use crate::fixtures::VIRT_GICV3;

fn fdt() -> Fdt<'static> {
    Fdt::from_bytes(VIRT_GICV3).unwrap()
}

#[test]
fn memory_and_cpus() {
    let mut ranges = [MemoryRange { base: 0, size: 0 }; 4];
    assert_eq!(memory::find(&fdt(), &mut ranges), Ok(1));
    assert_eq!((ranges[0].base, ranges[0].size), (0x4000_0000, 0x2000_0000));
    let mut ids = [0u64; 4];
    assert_eq!(cpus::find(&fdt(), &mut ids), Ok(2));
    assert_eq!(&ids[..2], &[0, 1]);
}

#[test]
fn console_gic_and_timer() {
    let u = uart::find(&fdt()).unwrap().unwrap();
    assert_eq!((u.kind, u.base), (uart::UartKind::Pl011, 0x0900_0000));
    // The clock comes through `clocks` from virt's fixed 24 MHz APB clock.
    assert_eq!((u.clock_hz, u.baud), (Some(24_000_000), None));
    let g = gic::find(&fdt()).unwrap().unwrap();
    assert_eq!(g.version, gic::GicVersion::V3);
    assert_eq!((g.dist_base, g.redist_or_cpu_base), (0x0800_0000, 0x080a_0000));
    let t = timer::find(&fdt()).unwrap().unwrap();
    assert_eq!((t.secure_phys_intid, t.nonsecure_phys_intid), (29, 30));
    assert_eq!((t.virtual_intid, t.hyp_phys_intid), (27, 26));
}

#[test]
fn pci_host_and_rtc() {
    let p = pci::find(&fdt()).unwrap().unwrap();
    assert_eq!((p.ecam_base, p.ecam_size), (0x40_1000_0000, 0x1000_0000));
    assert_eq!((p.io_cpu_base, p.io_port_base, p.io_size), (0x3eff_0000, 0, 0x1_0000));
    assert_eq!((p.mem_cpu_base, p.mem_bus_base), (0x1000_0000, 0x1000_0000));
    assert_eq!((p.mem_size, p.bus_start, p.bus_end), (0x2eff_0000, 0, 255));
    assert_eq!(rtc::find(&fdt()), Ok(Some(0x0901_0000)));
}

#[test]
fn chosen_and_reservations() {
    let f = fdt();
    let (node, options) = f.stdout().unwrap().unwrap();
    assert_eq!((node.name, options), (&b"pl011@9000000"[..], &b""[..]));
    assert_eq!(f.chosen_bytes(b"rng-seed").unwrap().map(<[u8]>::len), Some(32));
    assert_eq!(f.chosen_bytes(b"kaslr-seed").unwrap().map(<[u8]>::len), Some(8));
    assert_eq!(f.chosen_bytes(b"bootargs"), Ok(None));
    assert_eq!(f.initrd(), Ok(None));
    assert_eq!(f.reservations().count(), 0);
    assert_eq!(f.reserved_memory(&mut []), Ok(0));
}

#[test]
fn every_virtio_slot_is_a_platform_device_with_its_spi() {
    let f = fdt();
    let found: Vec<_> = f.devices(&[b"virtio,mmio"]).map(Result::unwrap).collect();
    assert_eq!(found.len(), 32);
    for (i, (node, _)) in found.iter().enumerate() {
        let base = 0x0a00_0000 + 0x200 * i as u64;
        assert_eq!(f.mmio(node, 0), Ok(Some((base, 0x200))));
        assert_eq!(f.mmio(node, 1), Ok(None));
        let spec = f.interrupt(node, 0).unwrap().unwrap();
        assert_eq!(spec.controller.name, b"intc@8000000");
        assert_eq!(crate::arch::fdt::irq::gic_intid(spec.cells), Ok(48 + i as u32));
        assert_eq!(crate::arch::fdt::irq::gic_edge(spec.cells), Ok(true));
        assert!(f.interrupt(node, 1).unwrap().is_none());
    }
}
