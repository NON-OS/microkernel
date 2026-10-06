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
use super::layout::Queue;
use crate::constants::STATUS_OFFSET;
use core::ptr::read_volatile;
const USED_IDX_OFFSET: usize = 2;
impl Queue {
    pub fn used_idx(&self) -> u16 {
        unsafe { read_volatile(self.region_va.add(self.used_offset + USED_IDX_OFFSET).cast()) }
    }
    /// Requests handed to the device so far, as the avail ring counts them.
    pub fn avail_idx(&self) -> u16 {
        unsafe { read_volatile(self.region_va.add(self.avail_offset + 2).cast()) }
    }
    /// Whether the device has answered every request it was given. One
    /// slot and one data buffer serve them all, so neither may be touched
    /// while a request is still out.
    pub fn idle(&self) -> bool {
        self.used_idx() == self.avail_idx()
    }
    pub fn status_byte(&self) -> u8 {
        unsafe { read_volatile(self.header_va.add(STATUS_OFFSET)) }
    }
    /// The first `len` bytes of the data buffer, at most its length.
    ///
    /// # Safety
    ///
    /// The device must have answered every request (`idle`), or it may still
    /// be writing the buffer under the slice.
    pub unsafe fn data(&self, len: u32) -> &[u8] {
        let n = core::cmp::min(len, self.data_len) as usize;
        core::slice::from_raw_parts(self.data_va, n)
    }
    /// The first `len` bytes of the data buffer, to fill before a write.
    ///
    /// # Safety
    ///
    /// The device must have answered every request (`idle`), or it may still
    /// be reading the buffer for a write being changed under it.
    pub unsafe fn data_mut(&mut self, len: u32) -> &mut [u8] {
        let n = core::cmp::min(len, self.data_len) as usize;
        core::slice::from_raw_parts_mut(self.data_va, n)
    }
}
