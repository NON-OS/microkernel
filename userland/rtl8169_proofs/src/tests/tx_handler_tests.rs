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
//! frame and padded to the 60-byte wire minimum; a next slot still the
//! part's answers "again" while the link is down; and once it has stayed
//! the part's past the bound with the link up, the next send resets the part
//! and goes out.

use std::sync::{atomic::AtomicU32, atomic::Ordering::SeqCst, Arc};

use nonos_libc::entropy;

use super::memory::Memory;
use super::model::window;
use super::serve::{ask, serve, status};
use super::tx_hang_tests::stalled;
use crate::constants::regs::{DESC_LEN_MASK, DESC_OWN};
use crate::protocol::{E_AGAIN, E_INVAL, OP_TX_PACKET};
use crate::queue::desc::{desc, desc_mut, Descriptor};

#[test]
fn bodies_at_and_past_every_bound_and_short_ones_padded() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    for (len, want) in [(0, E_INVAL), (13, E_INVAL), (14, 0), (1514, 0)] {
        let got = serve(&mut d, &[ask(OP_TX_PACKET, &vec![0xA5; len])]);
        assert_eq!(status(&got[0].1), want, "{len}-byte body");
    }
    // SAFETY: slots 0 and 1 are inside the ring `mem` holds.
    let sent = unsafe { [desc(d.tx.desc_va, 0), desc(d.tx.desc_va, 1)] };
    assert_eq!(
        sent.map(|s| s.opts1 & (DESC_OWN | DESC_LEN_MASK)),
        [DESC_OWN | 60, DESC_OWN | 1514]
    );
    assert!(mem.tx_buf[..14].iter().all(|b| *b == 0xA5), "the frame is in slot 0");
    assert!(mem.tx_buf[14..60].iter().all(|b| *b == 0), "and padded with zeros");
    // The part still holding the next slot, with the link down, is a full ring.
    let owned = Descriptor { opts1: DESC_OWN, opts2: 0, addr_lo: 0, addr_hi: 0 };
    // SAFETY: as above.
    unsafe { desc_mut(d.tx.desc_va, 2, owned) };
    let full = serve(&mut d, &[ask(OP_TX_PACKET, &[1; 60])]);
    assert_eq!((status(&full[0].1), d.tx.cur), (E_AGAIN, 2));
}

#[test]
fn a_slot_held_past_the_bound_with_the_link_up_is_reset_by_the_next_send() {
    let (_turn, resets) = (entropy(true), Arc::new(AtomicU32::new(0)));
    let (_bar, _part, mut d, _mem) = stalled(&resets);
    let done = resets.load(SeqCst);
    let got = serve(&mut d, &[ask(OP_TX_PACKET, &[2; 60])]);
    assert_eq!(status(&got[0].1), 0, "the reset made room and the frame went");
    assert_eq!((resets.load(SeqCst), d.tx.cur), (done + 1, 1));
}
