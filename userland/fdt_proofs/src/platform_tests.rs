// NONOS Operating System (AGPL-3.0-or-later)
//! Platform devices from small trees: `dma-ranges` narrowed bus by bus, and
//! a node whose interrupt or windows cannot be served left unpublished.

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::find::platform::{find, PlatformDevice, PlatformIrq};
use crate::arch::fdt::{DmaWindow, Fdt};
use crate::builder::{cells, text, Builder, END};

/// Root (2/1 cells) with a GIC (phandle 1), bus `outer` (1/1) holding bus
/// `inner` (1/1) holding one SDHCI. Each bus's `dma-ranges` is given raw;
/// `None` leaves it off.
pub(crate) fn tree(outer: Option<&[u32]>, inner: Option<&[u32]>, irq: &[u32]) -> Vec<u8> {
    let mut b = Builder::default().begin("");
    b = b.prop("#address-cells", &cells(&[2])).prop("#size-cells", &cells(&[1]));
    b = b.prop("interrupt-parent", &cells(&[1]));
    b = b.begin("gic").prop("compatible", &text("arm,gic-400"));
    b = b.prop("#interrupt-cells", &cells(&[3])).prop("phandle", &cells(&[1])).end();
    b = b.begin("outer").prop("#address-cells", &cells(&[1])).prop("#size-cells", &cells(&[1]));
    b = b.prop("ranges", &cells(&[0, 0, 0, 0x8000_0000]));
    if let Some(d) = outer {
        b = b.prop("dma-ranges", &cells(d));
    }
    b = b.begin("inner").prop("#address-cells", &cells(&[1])).prop("#size-cells", &cells(&[1]));
    b = b.prop("ranges", &[]);
    if let Some(d) = inner {
        b = b.prop("dma-ranges", &cells(d));
    }
    b = b.begin("mmc@1000").prop("compatible", &text("brcm,bcm2711-emmc2"));
    b = b.prop("reg", &cells(&[0x1000, 0x100])).prop("interrupts", &cells(irq));
    b.end().end().end().end().token(END).finish()
}

pub(crate) fn one(blob: &[u8]) -> Result<Option<PlatformDevice>, FdtError> {
    let mut out = [PlatformDevice::EMPTY; 2];
    let n = find(&Fdt::from_bytes(blob).unwrap(), &mut out)?;
    Ok(out[..n].first().copied())
}

pub(crate) const SPI_9_LEVEL: &[u32] = &[0, 9, 4];

#[test]
fn absent_and_empty_dma_ranges_are_one_to_one() {
    let d = one(&tree(None, Some(&[]), SPI_9_LEVEL)).unwrap().unwrap();
    assert_eq!(d.dma, Some(DmaWindow::IDENTITY));
    assert_eq!(d.irq, Some(PlatformIrq { intid: 41, edge: false }));
    assert_eq!((d.window_count, d.windows[0]), (1, (0x1000, 0x100)));
}

#[test]
fn each_bus_narrows_and_moves_the_window() {
    // inner: its bus 0x8000_0000.. is outer's 0x1000_0000.. for 1 GiB.
    // outer: its 0x0.. is the root's 0x4000_0000.. for 512 MiB.
    let inner: &[u32] = &[0x8000_0000, 0x1000_0000, 0x4000_0000];
    let outer: &[u32] = &[0, 0, 0x4000_0000, 0x2000_0000];
    let d = one(&tree(Some(outer), Some(inner), SPI_9_LEVEL)).unwrap().unwrap();
    // Only outer's first 512 MiB exist, and inner starts 256 MiB into them.
    let want = DmaWindow { bus_base: 0x8000_0000, cpu_base: 0x5000_0000, size: 0x1000_0000 };
    assert_eq!(d.dma, Some(want));
}

#[test]
fn the_widest_entry_wins_and_disjoint_buses_give_no_dma() {
    let outer: &[u32] = &[0x7c00_0000, 0, 0xfc00_0000, 0x0380_0000, 0, 0, 0, 0x4000_0000];
    let d = one(&tree(Some(outer), None, SPI_9_LEVEL)).unwrap().unwrap();
    assert_eq!(d.dma, Some(DmaWindow { bus_base: 0, cpu_base: 0, size: 0x4000_0000 }));
    let inner: &[u32] = &[0, 0x9000_0000, 0x1000];
    let outer: &[u32] = &[0, 0, 0, 0x1000_0000];
    assert_eq!(one(&tree(Some(outer), Some(inner), SPI_9_LEVEL)).unwrap().unwrap().dma, None);
}
