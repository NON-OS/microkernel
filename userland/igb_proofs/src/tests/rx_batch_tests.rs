// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! OP_RX_BATCH as net.core reads it, and the counters past it (R12).

use super::memory::Memory;
use super::model::window;
use super::net_reader::{batch_frames, served};
use super::serve::{ask, pools, serve, status};
use crate::facts::regs::REG_RDT;
use crate::facts::rx::{RXDEXT_ERR_FRAME_ERR_MASK as RXE, RXD_STAT_DD as DD, RXD_STAT_EOP};
use crate::protocol::{E_AGAIN, OP_RX_BATCH, OP_STATS, RESP_HDR_LEN, STATUS_LEN};
use crate::queue::layout::RxDesc;
use crate::sizes::ring::RX_BUFFER_LEN;

const DONE: u32 = DD | RXD_STAT_EOP;
const BAD: u32 = DONE | RXE;
const AT: usize = RESP_HDR_LEN + STATUS_LEN;

fn net_cores_batch_op() -> u16 {
    include_str!("../../../capsule_net_core/src/protocol/ops.rs")
        .lines()
        .find_map(|l| l.strip_prefix("pub const OP_RX_BATCH: u16 = "))
        .and_then(|v| v.trim_end_matches(';').parse::<u16>().ok())
        .expect("net.core names its batch op")
}

#[test]
fn the_counters_are_not_on_net_cores_batch_op() {
    assert_ne!(OP_STATS, net_cores_batch_op());
}

#[test]
fn the_probe_is_answered_with_the_empty_batch_net_core_takes() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    let got = serve(&mut d, &[ask(OP_RX_BATCH, &[])]);
    assert_eq!(status(&got[0].1), 0);
    assert!(served(Some(&got[0].1[AT..])), "net.core reads it as batches served");
}

#[test]
fn good_frames_come_in_order_and_every_slot_goes_back() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    let (mut pool, _tx) = pools(&mut d);
    for (slot, len, staterr) in [(0, 60, DONE), (1, 1514, DONE), (2, 60, BAD), (3, 14, DONE)] {
        pool[slot * RX_BUFFER_LEN..][..len].fill(slot as u8 + 1);
        mem.rx[slot] = RxDesc { buffer_addr: 0, staterr, length: len as u16, vlan: 0 };
    }
    let (one, two) = (1u32.to_le_bytes(), 2u32.to_le_bytes());
    let got =
        serve(&mut d, &[ask(OP_RX_BATCH, &one), ask(OP_RX_BATCH, &one), ask(OP_RX_BATCH, &two)]);
    let frames = batch_frames(&got[0].1[AT..]).expect("net.core reads the batch whole");
    assert_eq!(frames, [vec![1u8; 60], vec![2; 1514], vec![4; 14]], "slot 2 errored, left out");
    assert_eq!(bar.wrote32(REG_RDT), 3, "the last slot taken, after its copy");
    assert!(mem.rx[..4].iter().all(|r| r.staterr == 0), "all four back in read format");
    assert_eq!(got[1].1, got[0].1, "the same number is the same batch");
    assert_eq!(status(&got[2].1), E_AGAIN, "nothing new");
}
