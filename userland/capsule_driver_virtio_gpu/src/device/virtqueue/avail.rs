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

//! The available ring, which the driver writes.

use core::sync::atomic::{fence, Ordering};

use super::layout::QueueLayout;
use crate::constants::VQ_AVAIL_OFFSET;

const AVAIL_IDX: usize = VQ_AVAIL_OFFSET + 2;
const AVAIL_RING: usize = VQ_AVAIL_OFFSET + 4;

/// Offer chain `head` as available entry `next`, then publish the index.
/// `next` is the driver's own count: the copy in shared memory is the
/// device's to read, not the driver's to trust.
pub fn publish(layout: QueueLayout, next: u16, head: u16) {
    layout.ring.w16(AVAIL_RING + 2 * usize::from(next % layout.queue_size), head);
    // The device may read the entry, its chain and the request the moment
    // the index covers them.
    fence(Ordering::Release);
    layout.ring.w16(AVAIL_IDX, next.wrapping_add(1));
}
