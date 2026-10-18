// NONOS Operating System (AGPL-3.0-or-later)
//! The firmware mailbox, as a Pi 4's tree places it: under `/soc`, whose
//! `ranges` move its registers and whose `dma-ranges` put RAM at 0xc000_0000.

use crate::arch::fdt::find::mailbox::{find, MailboxInfo};
use crate::arch::fdt::{DmaWindow, Fdt};
use crate::builder::{cells, text, Builder, END};

fn tree(status: &str, dma: &[u32]) -> Vec<u8> {
    let mut b = Builder::default().begin("");
    b = b.prop("#address-cells", &cells(&[2])).prop("#size-cells", &cells(&[1]));
    b = b.begin("soc").prop("compatible", &text("simple-bus"));
    b = b.prop("#address-cells", &cells(&[1])).prop("#size-cells", &cells(&[1]));
    b = b.prop("ranges", &cells(&[0x7e00_0000, 0, 0xfe00_0000, 0x0180_0000]));
    b = b.prop("dma-ranges", &cells(dma));
    b = b.begin("mailbox@7e00b880").prop("compatible", &text("brcm,bcm2835-mbox"));
    b = b.prop("reg", &cells(&[0x7e00_b880, 0x40])).prop("status", &text(status));
    b.end().end().end().token(END).finish()
}

const PI4_DMA: &[u32] = &[0xc000_0000, 0, 0, 0x4000_0000];

#[test]
fn the_pi_mailbox_and_the_bus_its_buffers_are_named_on() {
    let blob = tree("okay", PI4_DMA);
    let found = find(&Fdt::from_bytes(&blob).unwrap()).unwrap();
    let dma = DmaWindow { bus_base: 0xc000_0000, cpu_base: 0, size: 0x4000_0000 };
    assert_eq!(found, Some(MailboxInfo { base: 0xfe00_b880, size: 0x40, dma }));
    assert_eq!(dma.bus_address(0x8_0000, 0x100), Some(0xc008_0000));
}

#[test]
fn a_disabled_mailbox_or_one_no_bus_reaches_is_none() {
    let blob = tree("disabled", PI4_DMA);
    assert_eq!(find(&Fdt::from_bytes(&blob).unwrap()), Ok(None));
    // The root's address cells hold no RAM the bus maps: a window of zero size.
    let blob = tree("okay", &[0, 0, 0, 0]);
    assert_eq!(find(&Fdt::from_bytes(&blob).unwrap()), Ok(None));
}
