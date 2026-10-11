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

//! What init leaves on the standard ring: every BD sized and marked as
//! tg3_init_rx_ring does, and RX_BUFFERS of them pointing at their slots.

use crate::constants::desc::*;
use crate::constants::layout::*;
use crate::constants::ring::*;

use super::memory::{Memory, RX_BUF_PHYS};

#[test]
fn init_posts_every_buffer_once() {
    let mut m = Memory::new();
    let mut q = m.rx_queue();
    q.init();
    assert_eq!(q.std_prod as usize, RX_BUFFERS);
    for slot in 0..RX_STD_RING_SIZE {
        let bd = STD_RING_AT + slot * RX_BD_BYTES;
        assert_eq!(m.get32(bd + 8), RX_STD_DMA_SZ, "len {slot}");
        assert_eq!(m.get32(bd + 12), RXD_FLAG_END);
        assert_eq!(m.get32(bd + 28), RXD_OPAQUE_RING_STD | slot as u32);
        let at = (m.get32(bd) as u64) << 32 | m.get32(bd + 4) as u64;
        let want = RX_BUF_PHYS + (slot * RX_BUFFER_LEN) as u64;
        assert_eq!(at, if slot < RX_BUFFERS { want } else { 0 }, "addr {slot}");
    }
}
