// NONOS Operating System (AGPL-3.0-or-later)
//! OP_RX_BATCH as net.core reads it: the empty probe is the one yes
//! net.core takes, every good frame comes in order, an errored slot is left
//! out but given back, and a batch asked for again is the same batch.

use super::memory::Memory;
use super::model::window;
use super::net_reader::{batch_frames, served};
use super::serve::{ask, pools, serve, status};
use crate::constants::queue::RX_BUFFER_LEN;
use crate::constants::regs::REG_RDT;
use crate::constants::rx_bits::{RXDEXT_STATERR_RXE as RXE, RXD_STAT_DD as DD, RXD_STAT_EOP};
use crate::protocol::{E_AGAIN, OP_RX_BATCH, RESP_HDR_LEN, STATUS_LEN};
use crate::queue::layout::RxDesc;

const DONE: u32 = DD | RXD_STAT_EOP;

fn body(reply: &[u8]) -> &[u8] {
    &reply[RESP_HDR_LEN + STATUS_LEN..]
}

#[test]
fn the_probe_is_answered_with_the_empty_batch_net_core_takes() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    let got = serve(&mut d, &[ask(OP_RX_BATCH, &[])]);
    assert_eq!(status(&got[0].1), 0);
    assert!(served(Some(body(&got[0].1))), "net.core reads it as batches served");
}

#[test]
fn good_frames_come_in_order_and_every_slot_goes_back() {
    let bar = window();
    bar.present32(REG_RDT, 31);
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    let (mut pool, _tx) = pools(&mut d);
    for (slot, len, staterr) in [(0, 60, DONE), (1, 1514, DONE), (2, 60, DONE | RXE), (3, 14, DONE)]
    {
        pool[slot * RX_BUFFER_LEN..][..len].fill(slot as u8 + 1);
        mem.rx[slot] = RxDesc { buffer_addr: 0, staterr, length: len as u16, vlan: 0 };
    }
    let (one, two) = (1u32.to_le_bytes(), 2u32.to_le_bytes());
    let got =
        serve(&mut d, &[ask(OP_RX_BATCH, &one), ask(OP_RX_BATCH, &one), ask(OP_RX_BATCH, &two)]);
    let frames = batch_frames(body(&got[0].1)).expect("net.core reads the batch whole");
    assert_eq!(frames, [vec![1u8; 60], vec![2; 1514], vec![4; 14]], "slot 2 errored, left out");
    assert_eq!(bar.wrote32(REG_RDT), 3, "the last slot taken, after its copy");
    assert!(mem.rx[..4].iter().all(|r| r.staterr == 0), "all four back in read format");
    assert_eq!(got[1].1, got[0].1, "the same number is the same batch");
    assert_eq!(status(&got[2].1), E_AGAIN, "nothing new");
}
