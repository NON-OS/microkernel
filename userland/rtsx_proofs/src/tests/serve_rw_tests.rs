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

//! READ, WRITE and FLUSH over the wire: blocks land on the card where
//! they were asked and read back as written; every range past the card or
//! the buffer is refused before the card is asked anything; and the data
//! buffer holds nothing once a request is answered (C8).

use super::rig::{sdhc, serving};
use super::service_rig::{call, span, KERNEL};
use crate::model::with;
use crate::protocol::*;

fn pattern(blocks: usize, seed: u8) -> Vec<u8> {
    (0..blocks * 512).map(|i| (i % 251) as u8 ^ seed).collect()
}

#[test]
fn blocks_written_are_on_the_card_and_read_back_as_written() {
    let (mut drv, mut medium) = serving(sdhc(1));
    for (lba, count) in [(100u64, 8usize), (7, 1), (0, 128)] {
        let data = pattern(count, lba as u8);
        let body = [span(lba, count as u32), data.clone()].concat();
        let w = call(&mut drv, &mut medium, KERNEL, OP_WRITE, &body);
        assert_eq!((w.status, w.payload.len()), (E_OK, 0));
        let on_card = with(|r| r.slot.as_ref().map(|c| c.block(lba + count as u64 - 1)));
        assert_eq!(on_card.as_deref(), Some(&data[(count - 1) * 512..]));
        let r = call(&mut drv, &mut medium, KERNEL, OP_READ, &span(lba, count as u32));
        assert_eq!((r.status, &r.payload), (E_OK, &data));
    }
    assert_eq!(call(&mut drv, &mut medium, KERNEL, OP_FLUSH, &[]).status, E_OK);
    assert!((0..128 * 512).all(|i| drv.data.read_u8(i) == 0), "data left in the buffer");
}

#[test]
fn a_range_past_the_card_or_the_buffer_never_reaches_the_card() {
    let (mut drv, mut medium) = serving(sdhc(1));
    let end = 1u64 << 20;
    let refused = [
        (span(end - 1, 2), E_NXIO),
        (span(end, 1), E_NXIO),
        (span(u64::MAX, 1), E_NXIO),
        (span(u64::MAX - 1, 128), E_NXIO),
        (span(0, 0), E_INVAL),
        (span(0, 129), E_INVAL),
        (span(0, u32::MAX), E_INVAL),
        ([span(0, 1), vec![0]].concat(), E_MSGSIZE),
        (span(0, 1)[..11].to_vec(), E_MSGSIZE),
    ];
    for (body, status) in refused {
        with(|r| r.sent.clear());
        assert_eq!(call(&mut drv, &mut medium, KERNEL, OP_READ, &body).status, status);
        with(|r| assert!(r.sent.is_empty(), "the card was asked: {:?}", r.sent));
    }
    for body in [[span(0, 2), vec![0; 512]].concat(), [span(0, 1), vec![0; 513]].concat()] {
        assert_eq!(call(&mut drv, &mut medium, KERNEL, OP_WRITE, &body).status, E_MSGSIZE);
    }
    let last = call(&mut drv, &mut medium, KERNEL, OP_READ, &span(end - 128, 128));
    assert_eq!((last.status, last.payload.len()), (E_OK, 128 * 512));
}
