// NONOS Operating System (AGPL-3.0-or-later)
//! The control region holds everything without overlap, and the firmware
//! is packed into broker-sized, page-aligned blocks.

use crate::gen3::dram_map::classify;
use crate::gen3::layout::{MCR_SIZE, MTR_SIZE};
use crate::gen3::plan::*;
use crate::gen3::scan::SCAN_REQ_V17_LEN;
use crate::gen3_model::Mem;
use super::image::SO_GF;

#[test]
fn the_control_region_holds_everything_without_overlap() {
    let spans = [
        (CTXT_INFO, 104),
        (PRPH_SCRATCH, 1724),
        (PRPH_INFO, PAGE),
        (RB_STTS, 2),
        (RX_FREE, RX_RING * 16),
        (RX_USED, RX_RING * 32),
        (CMD_TFDS, CMD_RING * 256),
        (FIRST_TBS, CMD_RING * 64),
        (CMD_BUFFER, CMD_BUF),
        (IML, 13944),
    ];
    for w in spans.windows(2) {
        assert!(w[0].0 + w[0].1 <= w[1].0, "{:#x} runs into {:#x}", w[0].0, w[1].0);
    }
    for (off, _) in &spans[..8] {
        assert_eq!(off % PAGE, 0, "{off:#x} is page aligned");
    }
    const { assert!(CMD_BUF >= 8 + SCAN_REQ_V17_LEN, "the scan request fits the command buffer") };
    assert_eq!(control_len(13944), Some(0x19000));
    assert!(control_len(GRANT_MAX).is_none(), "an image loader too large for one grant");
    assert_eq!(RB_REGION, 64 * 2048);
    const { assert!(RB_REGION <= GRANT_MAX) };
    assert_eq!((MTR_SIZE, MCR_SIZE), (4, 9), "TFD_QUEUE_CB_SIZE(128), RX_QUEUE_CB_SIZE(512)");
}

#[test]
fn the_firmware_is_packed_into_broker_sized_page_aligned_blocks() {
    let layout = classify(SO_GF);
    let lens: Vec<usize> =
        layout.lmac.iter().chain(layout.umac.iter()).chain(layout.virt.iter()).map(|s| s.data.len()).collect();
    assert_eq!((layout.lmac.len(), layout.umac.len(), layout.virt.len()), (15, 16, 24));
    let plan = plan_firmware(&lens).expect("plans");
    assert!(plan.regions.iter().all(|&r| r <= GRANT_MAX && r % PAGE == 0));
    assert_eq!(plan.blocks.len(), lens.len());
    for (b, &len) in plan.blocks.iter().zip(&lens) {
        assert_eq!(b.len, len);
        assert_eq!(b.off % PAGE, 0, "each section on its own page");
        assert!(b.off + b.len <= plan.regions[b.region], "inside its region");
    }
    for w in plan.blocks.windows(2) {
        if w[0].region == w[1].region {
            assert!(w[0].off + w[0].len <= w[1].off, "no overlap");
        }
    }
    let total: usize = lens.iter().sum();
    assert!(plan.regions.iter().sum::<usize>() >= total);
    assert!(plan.regions.len() <= 8, "1.6 MB in at most eight 256 KiB grants");
    assert!(plan_firmware(&[0]).is_none(), "an empty section");
    assert!(plan_firmware(&[GRANT_MAX + 1]).is_none(), "a section larger than a grant");
}

pub(super) fn regions() -> (std::rc::Rc<Mem>, std::rc::Rc<Mem>) {
    (Mem::new(0x19000, 0x1000_0000), Mem::new(RB_REGION, 0x2000_0000))
}
