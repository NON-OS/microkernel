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

//! The send handler at the IPC boundary: a body is taken only between a bare
//! header and a whole Ethernet frame, a full ring answers "again", and once
//! a part that stopped sending with the link up has been stalled past the
//! bound, the next send resets it and goes out.

use nonos_libc::{entropy, set_command, Deadline, MK_PCI_CMD_MEMORY_SPACE};

use super::live::live_part;
use super::memory::Memory;
use super::model::window;
use super::serve::{ask, request, serve, status};
use crate::constants::queue::TX_DESC_COUNT;
use crate::constants::regs::{REG_STATUS, REG_TDT};
use crate::constants::status::STATUS_LU;
use crate::protocol::{E_AGAIN, E_INVAL, E_MSGSIZE, OP_TX_PACKET};

#[test]
fn bodies_at_and_past_every_bound() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    for (len, want) in [(0, E_INVAL), (13, E_INVAL), (14, 0), (1514, 0)] {
        let got = serve(&mut d, &[ask(OP_TX_PACKET, &vec![0xA5; len])]);
        assert_eq!(status(&got[0].1), want, "{len}-byte body");
    }
    assert_eq!((mem.tx[0].length, mem.tx[1].length, bar.wrote32(REG_TDT)), (14, 1514, 2));
    assert!(mem.tx_buf[..14].iter().all(|b| *b == 0xA5), "the frame is in its slot");
    // A body one byte past a frame does not fit the loop's buffer whole, and
    // a length that disagrees with the body is refused before any handler.
    let long = serve(&mut d, &[ask(OP_TX_PACKET, &[0; 1515])]);
    let liar = serve(&mut d, &[(7, request(OP_TX_PACKET, &[0; 60], 59))]);
    assert_eq!((status(&long[0].1), status(&liar[0].1)), (E_MSGSIZE, E_MSGSIZE));
    assert_eq!(bar.wrote32(REG_TDT), 2, "nothing refused was posted");
}

#[test]
fn a_full_ring_answers_again_until_the_watchdog_resets_a_stalled_part() {
    let _turn = entropy(true);
    let bar = window();
    let _part = live_part(&bar);
    set_command(MK_PCI_CMD_MEMORY_SPACE);
    let mut mem = Memory::new();
    let mut d = crate::init::finish(mem.driver(&bar)).expect("brought up");
    bar.present32(REG_STATUS, STATUS_LU);
    let sends: Vec<_> = (0..TX_DESC_COUNT).map(|_| ask(OP_TX_PACKET, &[1; 60])).collect();
    let codes: Vec<i32> = serve(&mut d, &sends).iter().map(|r| status(&r.1)).collect();
    let mut want = vec![0; TX_DESC_COUNT - 1];
    want.push(E_AGAIN);
    assert_eq!(codes, want, "one slot always stays empty, and nothing completes");
    d.tx_watch = Deadline::after_ms(0);
    let got = serve(&mut d, &[ask(OP_TX_PACKET, &[2; 60])]);
    assert_eq!(status(&got[0].1), 0, "the reset made room and the frame went");
    assert_eq!((d.tx.clean, d.tx.tail, bar.wrote32(REG_TDT)), (0, 1, 1));
}
