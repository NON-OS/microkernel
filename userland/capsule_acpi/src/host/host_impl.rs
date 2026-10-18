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

//! The interpreter's host in the capsule; how each space is reached is `spaces`.

use super::state::CapsuleHost;
use super::{lock, region, say, tables};
use crate::events_io::PortIo;
use crate::sys::clock::{now_ms, sleep_ms, stall_us};
use alloc::vec::Vec;
use nonos_aml::exec::{Host, RegionAccess};
use nonos_aml::object::Object;

impl Host for CapsuleHost {
    fn sleep(&mut self, ms: u64) {
        sleep_ms(ms)
    }
    fn stall(&mut self, us: u64) {
        stall_us(us)
    }
    fn timer(&mut self) -> u64 {
        now_ms().saturating_mul(10_000) // 100 ns ticks, from a millisecond clock
    }
    fn debug(&mut self, value: &Object) {
        say::debug(value)
    }
    fn fatal(&mut self, kind: u8, code: u32, arg: u64) {
        say::fatal(kind, code, arg)
    }
    fn region_read(&mut self, a: &RegionAccess<'_>) -> Option<u64> {
        region::read(self, a)
    }
    fn region_write(&mut self, a: &RegionAccess<'_>, v: u64) -> bool {
        region::write(self, a, v)
    }
    fn transaction(
        &mut self,
        a: &RegionAccess<'_>,
        _: [u8; 3],
        _: Option<&[u8]>,
        _: &mut Vec<u8>,
    ) -> bool {
        super::refuse::refuse(self, a.space.0, a.base);
        false
    }
    fn gpio(&mut self, _: &[u8], pin: u32, _: u32, _: Option<u64>) -> Option<u64> {
        super::refuse::refuse(self, 8, u64::from(pin));
        None
    }
    fn table(&mut self, sig: &[u8], oem: &[u8], oem_table: &[u8]) -> Option<Vec<u8>> {
        tables::find(sig, oem, oem_table)
    }
    fn admit_table(&mut self, bytes: &[u8]) -> bool {
        tables::admit(bytes)
    }
    fn lock_word(&mut self, old: u32, new: u32) -> Option<u32> {
        lock::word(self, old, new)
    }
    fn lock_released(&mut self) {
        let _ = self.pm1_cnt.map(|cnt| cnt.release_lock(&mut PortIo));
    }
}
