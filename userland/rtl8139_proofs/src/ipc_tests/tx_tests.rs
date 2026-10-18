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

//! The send handler: a body is taken only between a bare header and a whole
//! frame, a short one is padded to the 60-byte wire minimum with zeros, and
//! a slot status the part will not give is an I/O error, not a send.

use nonos_libc::attach;

use super::serve::{ask, serve, status};
use crate::constants::dma::{TX_SLOT_BYTES, TX_SLOT_COUNT};
use crate::constants::regs::{REG_TXSTATUS0, TSD_ERTXTH_256};
use crate::part::{driver_over, part, Refused};
use crate::protocol::{E_INVAL, E_IO, OP_TX_PACKET};

#[test]
fn bodies_at_and_past_every_bound_and_short_ones_padded() {
    let _part = part(false);
    let mut pool = vec![0xEEu8; TX_SLOT_COUNT * TX_SLOT_BYTES];
    let mut d = driver_over(0);
    d.tx_user_va = pool.as_mut_ptr() as u64;
    for (len, want) in [(0, E_INVAL), (13, E_INVAL), (14, 0), (1514, 0)] {
        let got = serve(&mut d, &[ask(OP_TX_PACKET, &vec![0xA5; len])]);
        assert_eq!(status(&got[0].1), want, "{len}-byte body");
    }
    assert_eq!(d.pio.r32(REG_TXSTATUS0), Ok(60 | TSD_ERTXTH_256), "sent as 60 bytes");
    assert_eq!(d.pio.r32(REG_TXSTATUS0 + 4), Ok(1514 | TSD_ERTXTH_256));
    assert!(pool[..14].iter().all(|b| *b == 0xA5), "the frame is in slot 0");
    assert!(pool[14..60].iter().all(|b| *b == 0), "and padded with zeros");
    assert_eq!(d.tx_cur, 2);
}

#[test]
fn a_slot_status_the_part_will_not_give_is_an_io_error_not_a_send() {
    let _port = attach(Box::new(Refused));
    let mut d = driver_over(0);
    d.tx_cur = 1;
    let got = serve(&mut d, &[ask(OP_TX_PACKET, &[1; 60])]);
    assert_eq!((status(&got[0].1), d.tx_cur, d.tx_dirty), (E_IO, 1, 0));
}
