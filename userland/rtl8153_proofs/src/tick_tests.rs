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

//! The link as net.core finds it: net.core asks only for the link until
//! it reads up, and sends no receive request to a NIC it has not bound,
//! so the link must come up from link questions and the run loop's tick
//! alone, with traffic started and no bulk IN looked at.

use std::time::Duration;

use nonos_usbnet::mock::MockBus;
use nonos_usbnet::nnet::{answer, Request, Stats, OP_LINK_STATUS};
use nonos_usbnet::Nic;

use crate::chip::{bound, PLA};
use crate::r8153::Rtl8153;

const PHYSTATUS: u16 = 0xe908;

/// One OP_LINK_STATUS answered, then the tick the run loop gives after
/// every answer; the link byte of the reply.
fn ask_link(nic: &mut Rtl8153<MockBus>) -> u8 {
    let req = Request { op: OP_LINK_STATUS, flags: 0, request_id: 1, payload_len: 0 };
    let mut tx = [0u8; 64];
    answer(Some(&mut *nic), &mut Stats::default(), &req, &[], &mut tx);
    let _ = nic.tick();
    tx[24]
}

#[test]
fn the_link_comes_up_from_link_questions_alone_and_starts_traffic() {
    let (mut nic, bus, chip) = bound(0x5c10);
    bus.0.borrow_mut().bulk_in.push_back(Ok(Some(vec![0; 64])));
    chip.borrow_mut().regs.put(PLA, PHYSTATUS, &0x0013u16.to_le_bytes());
    let mut up = 0;
    for _ in 0..3 {
        up = ask_link(&mut nic);
        if up == 1 {
            break;
        }
        std::thread::sleep(Duration::from_millis(1_050));
    }
    assert_eq!(up, 1, "net.core reads the link up and binds the NIC");
    assert_eq!(chip.borrow().regs.byte(PLA, 0xe813) & 0x0c, 0x0c, "CR_RE and CR_TE");
    assert_eq!(bus.0.borrow().bulk_in.len(), 1, "no bulk IN was looked at");
}
