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

//! The probe net.core sends a newly bound NIC, operation 6 with no body,
//! against drivers that number operation 6 for something else. e1000,
//! e1000e, igc, RTL8139 and RTL8169 answer it with twelve counter words and
//! status 0, and the USB NIC core did too; each was read as a batch that
//! took in no frame. Only an empty batch says batches are served.

use nonos_libc::{serial, set_responder};

use crate::device::batch_call::call;
use crate::device::batch_frames::batch_frames;
use crate::device::batch_served::served;

/// A driver that answers every call with `status` and `body`.
fn driver(status: i32, body: Vec<u8>) {
    set_responder(move |_, req| {
        let mut r = req[..20].to_vec();
        r[16..20].copy_from_slice(&(4 + body.len() as u32).to_le_bytes());
        r.extend_from_slice(&status.to_le_bytes());
        r.extend_from_slice(&body);
        r
    });
}

fn counters(words: [u32; 12]) -> Vec<u8> {
    words.iter().flat_map(|w| w.to_le_bytes()).collect()
}

#[test]
fn a_driver_answering_with_its_counters_is_not_taken_to_serve_batches() {
    let _one = serial();
    let mut one_bogus = [0u32; 12];
    (one_bogus[0], one_bogus[1]) = (1, 40);
    let read_as = batch_frames(&counters(one_bogus)).map(|f| f.len());
    assert_eq!(read_as, Some(1), "tx_frames 1, tx_errors 40 reads as one 40-byte frame");
    for words in [[0u32; 12], one_bogus, [7; 12]] {
        driver(0, counters(words));
        let answer = call(4210, None);
        assert_eq!(answer.as_deref(), Some(&counters(words)[..]), "status 0, 48 bytes");
        assert!(!served(answer.as_deref()), "{words:?}");
    }
}

#[test]
fn only_an_empty_batch_says_batches_are_served() {
    let _one = serial();
    driver(0, vec![0; 4]);
    assert!(served(call(4204, None).as_deref()), "virtio-net's answer");
    for (status, body) in [(-22, vec![]), (0, vec![]), (0, vec![1, 0, 0, 0]), (0, vec![0; 8])] {
        driver(status, body.clone());
        assert!(!served(call(4204, None).as_deref()), "status {status}, {body:?}");
    }
}
