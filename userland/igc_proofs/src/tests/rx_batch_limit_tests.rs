// NONOS Operating System (AGPL-3.0-or-later)
//! Where a batch stops: at BATCH_MAX_FRAMES, under BATCH_MAX_BYTES, and
//! never past one turn of the ring, so the rest waits for the next number.

use super::memory::Memory;
use super::model::window;
use super::net_reader::batch_frames;
use super::serve::{ask, pools, serve};
use crate::constants::queue::{RX_BUFFER_LEN, RX_DESC_COUNT};
use crate::constants::rx_bits::{RXD_STAT_DD as DD, RXD_STAT_EOP};
use crate::protocol::{BATCH_MAX_BYTES, BATCH_MAX_FRAMES, OP_RX_BATCH, RESP_HDR_LEN, STATUS_LEN};
use crate::queue::layout::RxDesc;

#[test]
fn a_full_ring_comes_in_two_batches_each_within_its_limits() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    let (mut pool, _tx) = pools(&mut d);
    for slot in 0..RX_DESC_COUNT {
        pool[slot * RX_BUFFER_LEN..][..1514].fill(slot as u8);
        mem.rx[slot] = RxDesc { buffer_addr: 0, staterr: DD | RXD_STAT_EOP, length: 1514, vlan: 0 };
    }
    let mut seen = Vec::new();
    for seq in 1..=2u32 {
        let got = serve(&mut d, &[ask(OP_RX_BATCH, &seq.to_le_bytes())]);
        let body = &got[0].1[RESP_HDR_LEN + STATUS_LEN..];
        assert!(body.len() <= BATCH_MAX_BYTES, "batch {seq} is {} bytes", body.len());
        let frames = batch_frames(body).expect("whole");
        assert!(frames.len() <= BATCH_MAX_FRAMES, "batch {seq} holds {}", frames.len());
        seen.extend(frames.iter().map(|f| f[0]));
    }
    assert_eq!(seen, (0..RX_DESC_COUNT as u8).collect::<Vec<_>>(), "every frame, once, in order");
}
