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

use crate::ring_sync::AtomicU32;

/*
 * The shared header of a ring: the producer's published head (whose top
 * bit is the consumer's doorbell arm, `doorbell.rs`) and the consumer's
 * released tail, each on its own cache line so the two sides do not share
 * a line they both write.
 *
 * Both sides map this memory and either may be hostile to the other. No
 * size, offset or capacity is read from here: each side holds its own
 * capacity, and keeps its own index as the truth for its side.
 */
#[repr(C, align(64))]
pub struct RingHeader {
    pub(crate) head: AtomicU32,
    _head_line: [u8; 60],
    pub(crate) tail: AtomicU32,
    _tail_line: [u8; 60],
}

impl RingHeader {
    // Not const: a model checker's atomics are built at run time.
    pub fn new() -> Self {
        Self {
            head: AtomicU32::new(0),
            _head_line: [0; 60],
            tail: AtomicU32::new(0),
            _tail_line: [0; 60],
        }
    }
}

impl Default for RingHeader {
    fn default() -> Self {
        Self::new()
    }
}
