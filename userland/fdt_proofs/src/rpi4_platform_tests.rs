// NONOS Operating System (AGPL-3.0-or-later)
//! Raspberry Pi 4 platform devices: eMMC2 behind a bus that moves its DMA,
//! GENET behind one that does not.

use crate::arch::fdt::find::platform::{self, PlatformDevice, PlatformIrq, PlatformKind};
use crate::arch::fdt::{DmaWindow, Fdt};
use crate::fixtures::RPI4B;

#[test]
fn emmc2_and_genet_are_published_with_their_dma_windows() {
    let mut out = [PlatformDevice::EMPTY; 4];
    // The xHCI node is disabled on this board: its controller is behind PCIe.
    assert_eq!(platform::find(&Fdt::from_bytes(RPI4B).unwrap(), &mut out), Ok(2));
    let [emmc2, genet, ..] = out;
    assert_eq!(emmc2.kind, PlatformKind::Sdhci);
    assert_eq!((emmc2.window_count, emmc2.windows[0]), (1, (0xfe34_0000, 0x100)));
    assert_eq!(emmc2.irq, Some(PlatformIrq { intid: 32 + 0x7e, edge: false }));
    // emmc2bus puts RAM's first gigabyte at bus 0xc000_0000.
    let low_gib = DmaWindow { bus_base: 0xc000_0000, cpu_base: 0, size: 0x4000_0000 };
    assert_eq!(emmc2.dma, Some(low_gib));
    assert_eq!(low_gib.bus_address(0x1000, 0x1000), Some(0xc000_1000));
    assert_eq!(low_gib.bus_address(0x3fff_f000, 0x2000), None);
    assert_eq!(genet.kind, PlatformKind::GenetV5);
    assert_eq!(genet.windows[0], (0xfd58_0000, 0x1_0000));
    assert_eq!(genet.irq, Some(PlatformIrq { intid: 32 + 0x9d, edge: false }));
    // scb's widest entry is one to one over 16 GiB; its narrow one is the
    // registers' and is not where RAM is.
    assert_eq!(genet.dma, Some(DmaWindow { bus_base: 0, cpu_base: 0, size: 0x4_0000_0000 }));
}
