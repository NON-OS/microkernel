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

//! What the interpreter asks of the capsule around it.

use super::RegionAccess;
use crate::object::Object;
use alloc::vec::Vec;

pub trait Host {
    /// Sleep (section 19.6.128): give up the processor for `ms` milliseconds.
    fn sleep(&mut self, ms: u64);
    /// Stall (section 19.6.129): busy-wait `us` microseconds.
    fn stall(&mut self, us: u64);
    /// Timer (section 19.6.136): a monotonic count of 100 ns ticks.
    fn timer(&mut self) -> u64;
    /// A Store to the Debug object.
    fn debug(&mut self, value: &Object);
    /// Fatal's operands, reported before the evaluation stops.
    fn fatal(&mut self, kind: u8, code: u32, arg: u64);
    /// One register read of `a.width` bytes; None if the access was refused
    /// or failed, which faults the evaluation.
    fn region_read(&mut self, a: &RegionAccess<'_>) -> Option<u64>;
    /// One register write; false if refused or failed.
    fn region_write(&mut self, a: &RegionAccess<'_>, value: u64) -> bool;
    /// One SMBus, IPMI, GenericSerialBus or PCC transaction: `data` goes in
    /// and the handler's response comes back in it. `access` is the field's
    /// AccessType, AccessAttrib and AccessLength; `connection` its resource.
    fn transaction(
        &mut self,
        a: &RegionAccess<'_>,
        access: [u8; 3],
        connection: Option<&[u8]>,
        data: &mut Vec<u8>,
    ) -> bool;
    /// GPIO pins `pin` to `pin + count - 1` of the connection's pin table:
    /// read when `write` is None, else written.
    fn gpio(&mut self, connection: &[u8], pin: u32, count: u32, write: Option<u64>) -> Option<u64>;
    /// A table the platform holds, by signature and OEM ids, for
    /// DataTableRegion and LoadTable.
    fn table(&mut self, signature: &[u8], oem_id: &[u8], oem_table_id: &[u8]) -> Option<Vec<u8>>;
    /// Compare-and-swap on the FACS Global Lock word: returns the word found,
    /// which equals `old` when `new` was stored; None if there is no FACS.
    fn lock_word(&mut self, old: u32, new: u32) -> Option<u32>;
    /// Sets GBL_RLS, telling firmware a lock it waited for is free.
    fn lock_released(&mut self);
}
