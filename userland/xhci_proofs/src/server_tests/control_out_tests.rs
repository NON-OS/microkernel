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

//! A control request OUT through the server: its data stage follows the
//! ten setup bytes, wLength of them, and goes out as the client gave it.
//! The stage once went out as zeros, so a SET_CUR of a sampling rate set
//! 0 Hz. A body that does not carry exactly that stage is refused before
//! anything is queued.

use nonos_libc::dma_host;

use super::bench::Bench;
use crate::constants::{TRB_BYTES, TRB_DIR_IN, TRB_TYPE_DATA_STAGE};
use crate::protocol::{E_INVAL, OP_CONTROL_TRANSFER};
use crate::trb::read_volatile_at;

const HIGH_SPEED: u32 = 3;
/// SET_CUR of endpoint 1's sampling frequency, three bytes of it.
const SET_CUR: [u8; 10] = [0, 0, 0x22, 0x01, 0, 1, 0x01, 0, 3, 0];

fn ep0_at(b: &mut Bench, slot: u8) -> u64 {
    let res = b.mux.ctxs[0].driver.slots.resources_mut(slot, 8);
    res.map(|r| r.ep0.enqueue_phys()).expect("an addressed slot")
}

fn set_cur(slot: u8, data: &[u8]) -> Vec<u8> {
    let mut body = SET_CUR.to_vec();
    body[0] = slot;
    body.extend_from_slice(data);
    body
}

#[test]
fn an_out_request_sends_the_bytes_it_carries() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    let at = ep0_at(&mut b, slot);
    let r = b.call(OP_CONTROL_TRANSFER, &set_cur(slot, &48_000u32.to_le_bytes()[..3]));
    assert_eq!((r.status, &r.body[..]), (0, &[3u8, 0, 0x80, 0xBB, 0x00][..]), "48000 Hz");
    let host = dma_host(at + TRB_BYTES as u64).expect("the EP0 ring");
    // SAFETY: a TRB slot of the slot's EP0 ring, which it still holds.
    let data = unsafe { read_volatile_at(host) };
    assert_eq!(data.get_type(), TRB_TYPE_DATA_STAGE);
    assert_eq!((data.d2 & 0x1_FFFF, data.d3 & TRB_DIR_IN), (3, 0), "three bytes, OUT");
}

#[test]
fn a_body_without_exactly_its_data_stage_is_refused_before_anything_is_queued() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    let before = ep0_at(&mut b, slot);
    for data in [&[0x80, 0xBB][..], &[0x80, 0xBB, 0, 0][..]] {
        assert_eq!(b.call(OP_CONTROL_TRANSFER, &set_cur(slot, data)).status, E_INVAL);
    }
    let get_cur = [slot, 0, 0xA2, 0x81, 0, 1, 0x01, 0, 3, 0, 9];
    assert_eq!(b.call(OP_CONTROL_TRANSFER, &get_cur).status, E_INVAL, "an IN with bytes");
    assert_eq!(ep0_at(&mut b, slot), before, "no TRB queued");
}
