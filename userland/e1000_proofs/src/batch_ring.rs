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

//! An RX ring and its buffers in host memory, completed the way the part
//! completes a descriptor, for the batch proofs.

use crate::constants::queue::{RX_BUFFER_LEN, RX_DESC_COUNT, RX_STATUS_DD, RX_STATUS_EOP};
use crate::protocol::BATCH_MAX_BYTES;
use crate::queue::layout::RxDesc;
use crate::queue::RxRing;

pub struct Ring {
    desc: Box<[RxDesc; RX_DESC_COUNT]>,
    buf: Vec<u8>,
    ring: RxRing,
}

impl Ring {
    pub fn new() -> Self {
        let mut desc = Box::new([RxDesc::default(); RX_DESC_COUNT]);
        let mut buf = vec![0u8; RX_DESC_COUNT * RX_BUFFER_LEN];
        let ring = RxRing::new(desc.as_mut_ptr() as u64, buf.as_mut_ptr() as u64, 0x1000_0000);
        Self { desc, buf, ring }
    }

    /// The part wrote `frame` into slot `i` and marked it done.
    pub fn complete(&mut self, i: u16, frame: &[u8]) {
        let at = i as usize * RX_BUFFER_LEN;
        self.buf[at..at + frame.len()].copy_from_slice(frame);
        let d = &mut self.desc[i as usize];
        (d.length, d.status, d.errors) = (frame.len() as u16, RX_STATUS_DD | RX_STATUS_EOP, 0);
    }

    /// The part marked slot `i` done with a receive error.
    pub fn fail(&mut self, i: u16) {
        let d = &mut self.desc[i as usize];
        (d.length, d.status, d.errors) = (64, RX_STATUS_DD | RX_STATUS_EOP, 0x01);
    }

    pub fn take(&mut self, max_frames: usize, gave: &mut Vec<u16>) -> Option<Vec<u8>> {
        self.ring.take_batch(max_frames, BATCH_MAX_BYTES, |i| gave.push(i))
    }
}

/// The frames in a batch body, as net.core reads them.
pub fn body_frames(body: &[u8]) -> Option<Vec<Vec<u8>>> {
    let count = u32::from_le_bytes(body.get(..4)?.try_into().ok()?) as usize;
    let (mut at, mut frames) = (4, Vec::new());
    for _ in 0..count {
        let len = u32::from_le_bytes(body.get(at..at + 4)?.try_into().ok()?) as usize;
        frames.push(body.get(at + 4..at + 4 + len)?.to_vec());
        at += 4 + len;
    }
    (at == body.len()).then_some(frames)
}
