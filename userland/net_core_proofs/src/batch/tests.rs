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

//! A driver counts as a batch server only when it answers the empty probe
//! with an empty batch. One that answers operation 6 with a register
//! snapshot, as e1000 and the Realtek drivers did, stays on the frame-at-a-
//! time path instead of having every frame dropped as an unreadable batch.

use crate::device::batch_frames::batch_frames;
use crate::device::batch_served::served;

fn serves_batches(body: &[u8]) -> bool {
    served(Some(body))
}

/// e1000's OP_STATS reply body: twelve little-endian counters.
fn snapshot(rx_packets: u32) -> Vec<u8> {
    let words = [rx_packets, 0, 7, 0, 0, 0, 1, 0, 0, 0, 0, 0];
    words.iter().flat_map(|w| w.to_le_bytes()).collect()
}

#[test]
fn an_empty_batch_is_a_batch_server() {
    assert!(serves_batches(&0u32.to_le_bytes()));
}

#[test]
fn a_register_snapshot_is_not_a_batch_server() {
    for rx in [0, 1, 3, 1000] {
        assert!(!serves_batches(&snapshot(rx)), "rx_packets {rx}");
    }
}

// What the old probe let through: read as a batch, the snapshot is either
// unreadable (so the frames were dropped) or, with no packets yet, an empty
// batch whose trailing counters fail the length check.
#[test]
fn a_snapshot_read_as_a_batch_yields_no_frames() {
    for rx in [0, 1, 3, 1000] {
        assert_eq!(batch_frames(&snapshot(rx)), None, "rx_packets {rx}");
    }
}

#[test]
fn anything_but_four_zero_bytes_is_refused() {
    for body in [&[][..], &[0, 0, 0], &[0, 0, 0, 0, 0], &[1, 0, 0, 0], &[0, 0, 0, 1]] {
        assert!(!serves_batches(body), "{body:?}");
    }
}
