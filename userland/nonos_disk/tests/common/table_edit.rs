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

//! A written primary table changed in place, its header and array then
//! sealed again with good CRCs, so a test changes one thing at a time and
//! the change is what a reader refuses. Mounted beside `crc` and `mem_disk`.

use nonos_disk::BlockSink;

use crate::crc::crc32;
use crate::mem_disk::MemDisk;

/// `change` gets the primary header's sector and the 128-entry array.
pub fn edit(disk: &mut MemDisk, change: impl FnOnce(&mut [u8], &mut [u8])) {
    let (mut h, mut a) = (disk.read_sectors(1, 1), disk.read_sectors(2, 32));
    change(&mut h, &mut a);
    h[88..92].copy_from_slice(&crc32(&a).to_le_bytes());
    h[16..20].fill(0);
    let own = crc32(&h[..92]);
    h[16..20].copy_from_slice(&own.to_le_bytes());
    disk.write_at(1, &h).unwrap();
    disk.write_at(2, &a).unwrap();
}

/// Add `by` to the little-endian u64 at `at`.
pub fn shift(b: &mut [u8], at: usize, by: i64) {
    let v = u64::from_le_bytes(b[at..at + 8].try_into().unwrap());
    b[at..at + 8].copy_from_slice(&v.wrapping_add_signed(by).to_le_bytes());
}
