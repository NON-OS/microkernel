// NONOS Operating System (AGPL-3.0-or-later)
//! Buffers the device gets back hold nothing of what they carried: a taken
//! receive buffer is cleared before it is posted again, and a transmit
//! buffer is cleared before a frame is written into it.

use super::regions::regions;
use super::rxq::complete_at;
use crate::gen3::cmdq::FIRST_TB;
use crate::gen3::region::Region;
use crate::gen3::rxq::{RxQueue, Taken};
use crate::gen3::txq::{TxQueue, TX_BUF, TX_REGION};
use crate::gen3_model::Mem;
use crate::regs::Mmio;

#[test]
fn a_taken_receive_buffer_is_cleared_before_it_goes_back() {
    let (ctrl, rbs) = regions();
    let mut q = RxQueue::new();
    assert!(q.init(ctrl.as_ref(), rbs.as_ref()));
    rbs.write(2 * 2048, &[0x5A; 2048]); // buffer 3
    rbs.write(3 * 2048, &[0x5A; 2048]); // buffer 4, not taken
    complete_at(&ctrl, 0, 3, 0, 1);
    let mut out = [0u8; 2048];
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::Buffer));
    assert_eq!(out, [0x5A; 2048], "the frame copied out first");
    let mut b = [0xFFu8; 2048];
    assert!(rbs.read(2 * 2048, &mut b) && b == [0; 2048], "then the buffer cleared");
    assert!(rbs.read(3 * 2048, &mut b) && b == [0x5A; 2048], "no other touched");
}

/// Registers that take every write and read zero.
struct Quiet;

impl Mmio for Quiet {
    fn read32(&self, _: usize) -> u32 {
        0
    }
    fn write32(&self, _: usize, _: u32) {}
}

#[test]
fn a_transmit_buffer_is_cleared_before_a_frame_goes_into_it() {
    let tx = Mem::new(TX_REGION, 0x8000_0000);
    assert!(tx.write(0, &vec![0xEE; TX_REGION]), "every byte left by earlier frames");
    let mut q = TxQueue::new(0);
    q.start(2, 0);
    let mut frame = vec![0xB0, 0x00, 0, 0];
    frame.extend_from_slice(&[0xAA; 18]);
    frame.extend_from_slice(&[0x10, 0x00, 1, 2, 3]);
    assert!(q.send(&Quiet, &*tx, &frame, 0));
    // Slot 0's descriptor names the buffer: its second TB starts FIRST_TB in.
    let mut a = [0u8; 8];
    assert!(tx.read(2 + 10 + 2, &mut a));
    let buf = (u64::from_le_bytes(a) - tx.dev()) as usize - FIRST_TB;
    let mut b = vec![0u8; TX_BUF];
    assert!(tx.read(buf, &mut b));
    assert!(!b.contains(&0xEE), "nothing earlier stays in the buffer");
    assert!(tx.read(buf + TX_BUF, &mut b) && b.iter().all(|&x| x == 0xEE), "the next is untouched");
}
