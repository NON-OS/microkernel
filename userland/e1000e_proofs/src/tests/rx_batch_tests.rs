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

//! OP_RX_BATCH as net.core reads it: counters past it, the empty probe a yes,
//! good frames in order, an errored slot given back, a repeat the same batch.

use super::net_reader::{batch_frames, served};
use super::serve::{ask, serve, status};
use crate::constants::queue::{RX_BUFFER_LEN, RX_STATUS_DD, RX_STATUS_EOP};
use crate::constants::regs::REG_RDT;
use crate::constants::Family;
use crate::model::memory::Memory;
use crate::model::window::i82574;
use crate::protocol::{E_AGAIN, OP_RX_BATCH, OP_STATS, RESP_HDR_LEN, STATUS_LEN};
use crate::queue::layout::RxDesc;

const DONE: u8 = RX_STATUS_DD | RX_STATUS_EOP;
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
    let bar = i82574();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar, Family::I82574, 0x10D3);
    let got = serve(&mut d, &[ask(OP_RX_BATCH, &[])]);
    assert_eq!(status(&got[0].1), 0);
    assert!(served(Some(&got[0].1[AT..])), "net.core reads it as batches served");
}

#[test]
fn good_frames_come_in_order_and_every_slot_goes_back() {
    let bar = i82574();
    bar.present32(REG_RDT, 31);
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar, Family::I82574, 0x10D3);
    for (slot, len, errors) in [(0, 60, 0), (1, 1514, 0), (2, 60, 0x80u8), (3, 14, 0)] {
        mem.rx_buf[slot * RX_BUFFER_LEN..][..len].fill(slot as u8 + 1);
        mem.rx[slot] = RxDesc { status: DONE, errors, length: len as u16, ..RxDesc::default() };
    }
    let (one, two) = (1u32.to_le_bytes(), 2u32.to_le_bytes());
    let got =
        serve(&mut d, &[ask(OP_RX_BATCH, &one), ask(OP_RX_BATCH, &one), ask(OP_RX_BATCH, &two)]);
    let frames = batch_frames(&got[0].1[AT..]).expect("net.core reads the batch whole");
    assert_eq!(frames, [vec![1u8; 60], vec![2; 1514], vec![4; 14]], "slot 2 errored, left out");
    assert_eq!(bar.wrote32(REG_RDT), 3, "the last slot taken, after its copy");
    assert!(mem.rx[..4].iter().all(|r| r.status == 0), "all four back to the part");
    assert_eq!(got[1].1, got[0].1, "the same number is the same batch");
    assert_eq!(status(&got[2].1), E_AGAIN, "nothing new");
}
