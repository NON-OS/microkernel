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

//! Handing the device one request: the header, the chain, the available
//! entry, then the index that makes them the device's.

use core::sync::atomic::{fence, Ordering};

use super::chain::HEAD;
use super::data::wipe;
use super::direction::Direction;
use super::fault::Fault;
use super::layout::Queue;
use crate::constants::{HEADER_OFFSET, STATUS_OFFSET};

/// VirtqAvail: u16 flags, u16 idx, u16 ring[size].
const AVAIL_IDX: usize = 2;
const AVAIL_RING: usize = 4;
/// The status byte until the device writes it: no status it sends.
const STATUS_UNSET: u8 = 0xFF;

impl Queue {
    /// Post one request of `nsectors` from `lba`. Refused while an earlier
    /// request is the device's, since its completion would pass for this
    /// one's, and stalled so the server resets the device. A read's buffer
    /// is emptied first: a device that writes less than it was given leaves
    /// zeros, never the bytes of the last request, which may have been
    /// another client's.
    pub fn post(&mut self, dir: Direction, lba: u64, nsectors: u32) -> Result<(), Fault> {
        if !self.idle() {
            return Err(self.stall(Fault::Stalled));
        }
        let fits = |n: &u32| *n as usize <= self.data.view.len();
        let data = dir.data_bytes(nsectors).filter(fits).ok_or(Fault::Oversize)?;
        if dir == Direction::Read {
            wipe(self.data.view, data as usize);
        }
        let header = self.header.view;
        header.w32(HEADER_OFFSET, dir.req_type());
        header.w32(HEADER_OFFSET + 4, 0);
        header.w32(HEADER_OFFSET + 8, lba as u32);
        header.w32(HEADER_OFFSET + 12, (lba >> 32) as u32);
        header.w8(STATUS_OFFSET, STATUS_UNSET);
        self.write_chain(dir, data);
        let avail = self.avail();
        self.ring.w16(avail, 0);
        self.ring.w16(avail + AVAIL_RING + 2 * self.pos(self.posted), HEAD);
        self.expect = dir.writable(data);
        self.posted = self.posted.wrapping_add(1);
        // The device may read the entry, its chain and the header the moment
        // the index covers them: on a weakly ordered machine (the capsule
        // ships for aarch64) they must be visible first.
        fence(Ordering::Release);
        self.ring.w16(avail + AVAIL_IDX, self.posted);
        Ok(())
    }
}
