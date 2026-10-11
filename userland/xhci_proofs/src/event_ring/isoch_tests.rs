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

//! An isoch OUT stream over the driver's own rings: each batch is a TD per
//! packet in the free half of the buffer, the doorbell rung, and the batch
//! before waited for. That wait takes its own TD's event, drops what its
//! endpoint reports that no TD names, and parks other endpoints' events.

use nonos_libc::dma_host;

use super::fixture::{Fixture, SLOT, TRB};
use super::isoch_pipe::{pipe, send, trb_at, CC_SUCCESS, DCI};

#[test]
fn the_first_batch_is_a_td_per_packet_and_waits_for_nothing() {
    let mut fx = Fixture::new();
    let mut p = pipe(&fx);
    assert_eq!(send(&mut fx, &mut p, &[192, 192, 96], 0xA5), Ok(CC_SUCCESS));
    assert_eq!(fx.doorbells.wrote32(SLOT as usize * 4), DCI as u32, "the endpoint's doorbell");
    let base = p.buf.phys();
    for (i, (len, at)) in [(192, 0), (192, 192), (96, 384)].into_iter().enumerate() {
        let t = trb_at(p.ring.phys() + i as u64 * TRB);
        assert_eq!((t.get_pointer(), t.d2 & 0x1_FFFF), (base + at, len), "TD {i}");
        assert_eq!(((t.d3 >> 10) & 0x3F, t.d3 & (1 << 5) != 0), (5, i == 2), "TD {i}");
    }
    let host = dma_host(base).unwrap() as *const u8;
    // SAFETY: the buffer is two ISOCH_MAX halves of host memory.
    let first = unsafe { core::slice::from_raw_parts(host, 480) };
    assert!(first.iter().all(|&b| b == 0xA5), "the packets' bytes in the first half");
}
