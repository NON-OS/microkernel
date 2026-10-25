// NONOS Operating System (AGPL-3.0-or-later)
//! The memory-mapped timer's frame: the first enabled one with a number, a
//! window and a physical interrupt, its window translated through the
//! node's `ranges`, and damaged values refused rather than read short.

use crate::arch::fdt::find::timer_mem::{find, TimerFrame};
use crate::arch::fdt::{Fdt, FdtError};
use crate::builder::{cells, text, Builder, END};

/// One `frame@`: (frame-number, disabled, interrupts, reg offset).
type Frame<'a> = (Option<&'a [u8]>, bool, bool, Option<u32>);

fn tree(frames: &[Frame]) -> Vec<u8> {
    let mut b = Builder::default().begin("").prop("interrupt-parent", &cells(&[1]));
    b = b.prop("#address-cells", &cells(&[2])).prop("#size-cells", &cells(&[2]));
    b = b.begin("intc").prop("compatible", &text("arm,gic-400"));
    b = b.prop("interrupt-controller", &[]).prop("#interrupt-cells", &cells(&[3]));
    b = b.prop("phandle", &cells(&[1])).end();
    b = b.begin("timer@2a810000").prop("compatible", &text("arm,armv7-timer-mem"));
    b = b.prop("reg", &cells(&[0, 0x2a81_0000, 0, 0x1_0000]));
    b = b.prop("#address-cells", &cells(&[1])).prop("#size-cells", &cells(&[1]));
    b = b.prop("ranges", &cells(&[0, 0, 0x2a80_0000, 0x20_0000]));
    for (i, (number, disabled, irq, reg)) in frames.iter().enumerate() {
        b = b.begin(&format!("frame@{i}"));
        if let Some(number) = number {
            b = b.prop("frame-number", number);
        }
        if *disabled {
            b = b.prop("status", &text("disabled"));
        }
        if *irq {
            b = b.prop("interrupts", &cells(&[0, 26 + i as u32, 4, 0, 40 + i as u32, 4]));
        }
        if let Some(at) = reg {
            b = b.prop("reg", &cells(&[*at, 0x1000, at + 0x1_0000, 0x1000]));
        }
        b = b.end();
    }
    b.end().end().token(END).finish()
}

fn frame(frames: &[Frame]) -> Result<Option<TimerFrame>, FdtError> {
    find(&Fdt::from_bytes(&tree(frames)).unwrap())
}

#[test]
fn the_first_usable_frame_translated() {
    let one = cells(&[1]);
    let frames: [Frame; 4] = [
        (Some(&cells(&[0])), true, true, Some(0x2_0000)),
        (Some(&one), false, false, Some(0x3_0000)),
        (None, false, true, Some(0x4_0000)),
        (Some(&cells(&[3])), false, true, Some(0x5_0000)),
    ];
    let found = TimerFrame { number: 3, base: 0x2a85_0000, size: 0x1000, intid: 32 + 29 };
    assert_eq!(frame(&frames), Ok(Some(found)));
    assert_eq!(frame(&frames[..3]), Ok(None));
    assert_eq!(frame(&[(Some(&one), false, true, None)]), Ok(None));
}

#[test]
fn no_node_is_no_frame_and_damage_is_refused() {
    let virt = Fdt::from_bytes(crate::fixtures::VIRT_GICV3).unwrap();
    assert_eq!(find(&virt), Ok(None));
    for bad in [vec![], vec![0, 0, 1], cells(&[0, 1]), cells(&[8])] {
        let frames: [Frame; 1] = [(Some(&bad), false, true, Some(0x3_0000))];
        assert_eq!(frame(&frames), Err(FdtError::BadCells), "{bad:?}");
    }
}
