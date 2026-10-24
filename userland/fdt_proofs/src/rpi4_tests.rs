// NONOS Operating System (AGPL-3.0-or-later)
//! Raspberry Pi 4: devices sit on buses whose `ranges` move them, the GIC is
//! a GIC-400 and the PCIe bridge is not ECAM.

use crate::arch::fdt::find::memory::MemoryRange;
use crate::arch::fdt::find::{cpus, gic, memory, pci, rtc, timer, uart};
use crate::arch::fdt::Fdt;
use crate::fixtures::RPI4B;

fn fdt() -> Fdt<'static> {
    Fdt::from_bytes(RPI4B).unwrap()
}

#[test]
fn memory_reservations_and_cpus() {
    let f = fdt();
    let mut ranges = [MemoryRange { base: 0, size: 0 }; 4];
    assert_eq!(memory::find(&f, &mut ranges), Ok(2));
    assert_eq!((ranges[0].base, ranges[0].size), (0, 0x3b40_0000));
    assert_eq!((ranges[1].base, ranges[1].size), (0x4000_0000, 0xbc00_0000));
    assert_eq!(f.reservations().collect::<Vec<_>>(), [Ok((0, 0x1000))]);
    // The bootloader's nvram nodes are still the empty template.
    assert_eq!(f.reserved_memory(&mut []), Ok(0));
    let mut ids = [0u64; 8];
    assert_eq!(cpus::find(&f, &mut ids), Ok(4));
    assert_eq!(&ids[..4], &[0, 1, 2, 3]);
    // Firmware parks every core on its own spin-table word, not behind PSCI.
    let mut release = [None; 8];
    assert_eq!(cpus::release_addresses(&f, &mut release), Ok(4));
    assert_eq!(&release[..4], &[Some(0xd8), Some(0xe0), Some(0xe8), Some(0xf0)]);
}

#[test]
fn the_gic_400_is_found_through_soc_ranges() {
    let g = gic::find(&fdt()).unwrap().unwrap();
    assert_eq!(g.version, gic::GicVersion::V2);
    assert_eq!((g.dist_base, g.redist_or_cpu_base), (0xff84_1000, 0xff84_2000));
    let t = timer::find(&fdt()).unwrap().unwrap();
    assert_eq!((t.nonsecure_phys_intid, t.virtual_intid), (30, 27));
}

#[test]
fn console_is_the_mini_uart_at_firmwares_speed() {
    let f = fdt();
    let (node, options) = f.stdout().unwrap().unwrap();
    assert_eq!((node.name, options), (&b"serial@7e215040"[..], &b"115200n8"[..]));
    let u = uart::find(&f).unwrap().unwrap();
    assert_eq!((u.kind, u.base), (uart::UartKind::MiniUart, 0xfe21_5040));
    assert_eq!((u.reg_shift, u.io_width, u.baud), (2, 4, Some(115_200)));
    // Its clock is the VPU's, behind a controller, so the tree gives no rate
    // and the divisor firmware set has to stand.
    assert_eq!(u.clock_hz, None);
}

#[test]
fn scb_and_emmc2bus_devices_translate() {
    let f = fdt();
    let at = |p: &[u8]| f.mmio(&f.find_path(p).unwrap().unwrap(), 0);
    assert_eq!(at(b"/emmc2bus/mmc@7e340000"), Ok(Some((0xfe34_0000, 0x100))));
    assert_eq!(at(b"/scb/ethernet@7d580000"), Ok(Some((0xfd58_0000, 0x1_0000))));
    assert_eq!(at(b"/scb/pcie@7d500000"), Ok(Some((0xfd50_0000, 0x9310))));
    assert_eq!(at(b"/soc/serial@7e201000"), Ok(Some((0xfe20_1000, 0x200))));
}

#[test]
fn no_ecam_no_rtc_and_disabled_nodes_stay_hidden() {
    let f = fdt();
    assert!(pci::find(&f).unwrap().is_none());
    assert!(rtc::find(&f).unwrap().is_none());
    assert_eq!(f.devices(&[b"generic-xhci"]).count(), 0);
    assert_eq!(f.devices(&[b"brcm,bcm2711-genet-v5"]).count(), 1);
}
