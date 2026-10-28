// NONOS Operating System (AGPL-3.0-or-later)

//! A receive ring: every slot but one armed and handed over
//! (`mt76_dma_rx_fill`), a buffer taken only once the WFDMA set its done
//! bit, re-armed after, a frame longer than one buffer dropped whole (its
//! last piece too), and a buffer the DMA marked dropped.

use super::ring_tests::grant;
use crate::dma::{Ring, Rx};
use crate::model::rig::{chip, regs};

const REG: u32 = 0xd4500 + 2 * 16;

fn armed(n: u16) -> (Ring, crate::hw::Regs, crate::model::Shared) {
    let c = chip();
    let r = regs(&c);
    let mut q = Ring::new(grant(n as usize * 16), grant(n as usize * 256), 256, n, REG).unwrap();
    q.reset(&r);
    q.fill(&r);
    (q, r, c)
}

#[test]
fn all_slots_but_one_are_armed() {
    let (_, _, c) = armed(8);
    assert_eq!(c.borrow().reg(REG + 8), 7, "cpu_idx: seven slots given");
}

#[test]
fn nothing_is_taken_before_the_wfdma_is_done_and_each_slot_comes_back() {
    let (mut q, r, c) = armed(4);
    let mut out = [0u8; 256];
    assert!(q.take(&r, &mut out).is_none());
    for i in 0..10u8 {
        c.borrow_mut().push_rx(2, vec![i; 40]);
        assert!(matches!(q.take(&r, &mut out), Some(Rx::Frame(40))));
        assert_eq!(out[..40], [i; 40]);
    }
    assert!(q.take(&r, &mut out).is_none());
}

#[test]
fn a_frame_over_several_buffers_is_dropped_to_its_last_piece() {
    let (mut q, r, c) = armed(8);
    let mut out = [0u8; 256];
    c.borrow_mut().push_rx(2, vec![9u8; 600]);
    c.borrow_mut().push_rx(2, vec![7u8; 30]);
    let got: Vec<bool> =
        (0..4).map(|_| matches!(q.take(&r, &mut out), Some(Rx::Frame(_)))).collect();
    assert_eq!(got, [false, false, false, true], "three pieces dropped, then the next frame");
    assert_eq!(out[..30], [7u8; 30]);
}

#[test]
fn a_buffer_the_dma_marked_is_dropped_and_its_slot_comes_back() {
    let (mut q, r, c) = armed(4);
    let mut out = [0u8; 256];
    c.borrow_mut().push_rx(2, vec![1u8; 20]);
    let d = c.borrow().reg(REG) + 4;
    let ctrl = crate::model::mem::word(d);
    crate::model::mem::write_mem(d, &(ctrl | 1 << 14).to_le_bytes());
    assert!(matches!(q.take(&r, &mut out), Some(Rx::Dropped)));
    assert_eq!(c.borrow().reg(REG + 8), 0, "re-armed: cpu_idx moved on");
}
