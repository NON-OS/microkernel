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

//! Bulk streams (OP_CONFIGURE_STREAMS, 0x24): the stream context array,
//! and the endpoint context that names it (stream_transfer_tests.rs has a
//! transfer on a stream).

use super::fixture::{Fixture, BULK_IN_DCI};
use super::toggle_slot::{storage_slot, CONTEXT_SIZE};
use crate::controller::{write_stream_input, StreamPipe, MAX_LOG2};
use crate::dma::DmaRegion;

fn dw(region: &DmaRegion, byte: usize) -> u32 {
    // SAFETY: the tests read inside regions they allocated.
    unsafe { (region.as_mut_ptr::<u8>().add(byte) as *const u32).read_volatile() }
}

#[test]
fn each_stream_but_the_reserved_first_names_its_own_ring() {
    let fx = Fixture::new();
    let s = StreamPipe::new(&fx.pool, 2).expect("streams");
    assert_eq!((s.rings.len(), s.usable(), s.max_pstreams()), (3, 3, 1));
    assert_eq!((dw(&s.array, 0), dw(&s.array, 4)), (0, 0), "stream 0 is reserved");
    for (i, ring) in s.rings.iter().enumerate() {
        let at = ring.phys() | 1 << 1 | 1;
        assert_eq!(dw(&s.array, 16 * (i + 1)), at as u32, "SCT primary, cycle set");
        assert_eq!(dw(&s.array, 16 * (i + 1) + 4), (at >> 32) as u32);
    }
    assert!(
        StreamPipe::new(&fx.pool, 0).is_err() && StreamPipe::new(&fx.pool, MAX_LOG2 + 1).is_err()
    );
}

#[test]
fn the_endpoint_is_dropped_and_added_again_naming_the_array() {
    let fx = Fixture::new();
    let res = storage_slot(&fx);
    let s = StreamPipe::new(&fx.pool, 3).expect("streams");
    write_stream_input(&res.input_context, &res.output_context, CONTEXT_SIZE, BULK_IN_DCI, &s);
    let ctx = |c: usize, d: usize| dw(&res.input_context, c * CONTEXT_SIZE as usize + d * 4);
    assert_eq!((ctx(0, 0), ctx(0, 1)), (1 << BULK_IN_DCI, 1 | 1 << BULK_IN_DCI));
    let ep = BULK_IN_DCI as usize + 1;
    assert_eq!(ctx(ep, 0), 2 << 10 | 1 << 15, "MaxPStreams 2 (eight entries), linear");
    assert_eq!(ctx(ep, 1), 3 << 1 | 6 << 3 | 15 << 8 | 1024 << 16, "type, burst, mps kept");
    assert_eq!((ctx(ep, 2), ctx(ep, 3)), (s.array.phys() as u32, (s.array.phys() >> 32) as u32));
}
