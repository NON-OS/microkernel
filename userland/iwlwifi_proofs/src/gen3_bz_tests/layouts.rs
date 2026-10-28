// NONOS Operating System (AGPL-3.0-or-later)

//! The receive and transmit memory in the V2 layout: a used-ring entry is
//! `iwl_rx_completion_desc_bz` (four bytes, `rbid` 0, `flags` 2), the free
//! index goes through HBUS_TARG_WRPTR at queue 512, and a queue's byte
//! count table is `TFD_QUEUE_BC_SIZE_GEN3_BZ` (4096) entries, the buffers
//! after it a page later. A V1 queue keeps its old offsets.

use crate::gen3::csr::CsrLayout;
use crate::gen3::plan::{GRANT_MAX, RB_STTS, RX_USED};
use crate::gen3::region::Region;
use crate::gen3::rxq::{RxQueue, Taken};
use crate::gen3::txq::area::{bc_extra, tx_region, txq_stride};
use crate::gen3::txq::{TxQueue, BC_TABLE_BZ, TXQ_STRIDE, TX_BUF, TX_REGION};
use crate::gen3_model::{Mem, Model};

#[test]
fn a_v2_used_entry_is_four_bytes_and_the_index_goes_to_queue_512() {
    let (ctrl, rbs) = (Mem::new(0x20000, 0x1000_0000), Mem::new(64 * 2048, 0x2000_0000));
    let model = Model::new(ctrl.clone(), rbs.clone());
    model.s.borrow_mut().v2 = true;
    let mut q = RxQueue::new();
    q.csr = CsrLayout::V2;
    assert!(q.init(ctrl.as_ref(), rbs.as_ref()));
    assert_eq!(q.csr, CsrLayout::V2, "init keeps the layout");
    q.kick(&model);
    assert_eq!(model.s.borrow().widx, 64, "HBUS_TARG_WRPTR, queue 512");
    // Buffer 3 whole at entry 0, buffer 7 a fragment at entry 1.
    ctrl.write(RX_USED, &[3, 0, 0, 0, 7, 0, 1, 0]);
    ctrl.write(RB_STTS, &2u16.to_le_bytes());
    let mut out = vec![0u8; 2048];
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::Buffer));
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::Fragment));
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), None);
}

#[test]
fn a_v2_queue_area_holds_the_bz_byte_count_table() {
    assert_eq!((bc_extra(CsrLayout::V1), bc_extra(CsrLayout::V2)), (0, 4096));
    assert_eq!(txq_stride(CsrLayout::V1), TXQ_STRIDE);
    assert_eq!(tx_region(CsrLayout::V1), TX_REGION);
    assert_eq!(tx_region(CsrLayout::V2), 2 * (TXQ_STRIDE + 4096));
    assert!(tx_region(CsrLayout::V2) <= GRANT_MAX);
    const DEV: u64 = 0x8000_0000;
    let tx = Mem::new(tx_region(CsrLayout::V2), DEV);
    let mut q = TxQueue::nth(1, CsrLayout::V2);
    assert!(q.clear(&*tx));
    q.start(9, 5);
    let base = txq_stride(CsrLayout::V2);
    assert_eq!(q.bc(&*tx), DEV + (base + 128 * 256) as u64);
    let doorbells = Model::new(Mem::new(4096, 0), Mem::new(4096, 0));
    let mut frame = vec![0xB0, 0, 0, 0];
    frame.extend_from_slice(&[0xAA; 20]);
    frame.extend_from_slice(&[1; 40]);
    assert!(q.send(&doorbells, &*tx, &frame, 0x4000));
    let mut tfd = [0u8; 32];
    tx.read(base + 5 * 256, &mut tfd);
    let addr = |at: usize| u64::from_le_bytes(tfd[at..at + 8].try_into().unwrap());
    let first = base + 128 * 256 + BC_TABLE_BZ + 5 * 64;
    let buf = base + 128 * 256 + BC_TABLE_BZ + 128 * 64 + 5 * TX_BUF;
    assert_eq!(addr(4), DEV + first as u64, "the first buffer after the 8 KiB table");
    assert_eq!(addr(14), DEV + (buf + 20) as u64);
    let mut bc = [0u8; 2];
    tx.read(base + 128 * 256 + 5 * 2, &mut bc);
    assert_eq!(u16::from_le_bytes(bc), frame.len() as u16);
}
