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

//! The command descriptor blocks the layer sends (SPC-5, SBC-4, MMC-6).
//! Multi-byte fields are big endian.

pub const TEST_UNIT_READY: u8 = 0x00;
pub const INQUIRY: u8 = 0x12;
pub const MODE_SENSE_6: u8 = 0x1A;
pub const READ_CAPACITY_10: u8 = 0x25;
pub const READ_10: u8 = 0x28;
pub const WRITE_10: u8 = 0x2A;
pub const SYNCHRONIZE_CACHE_10: u8 = 0x35;
pub const READ_16: u8 = 0x88;
pub const WRITE_16: u8 = 0x8A;
pub const SYNCHRONIZE_CACHE_16: u8 = 0x91;
pub const SERVICE_ACTION_IN_16: u8 = 0x9E;
pub const SA_READ_CAPACITY_16: u8 = 0x10;
pub const REPORT_LUNS: u8 = 0xA0;

/// The unit serial number VPD page.
pub const VPD_SERIAL: u8 = 0x80;
/// MODE SENSE: every page, current values.
const ALL_PAGES: u8 = 0x3F;
/// MODE SENSE: no block descriptors.
const DBD: u8 = 1 << 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cdb {
    bytes: [u8; 16],
    len: usize,
}

impl Cdb {
    fn new(len: usize, op: u8) -> Self {
        let mut bytes = [0u8; 16];
        bytes[0] = op;
        Self { bytes, len }
    }

    fn put(mut self, at: usize, field: &[u8]) -> Self {
        self.bytes[at..at + field.len()].copy_from_slice(field);
        self
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

pub fn test_unit_ready() -> Cdb {
    Cdb::new(6, TEST_UNIT_READY)
}

/// The standard data, or with `page` that VPD page; `alloc` bytes at most.
pub fn inquiry(page: Option<u8>, alloc: u16) -> Cdb {
    let c = Cdb::new(6, INQUIRY).put(3, &alloc.to_be_bytes());
    match page {
        Some(p) => c.put(1, &[1]).put(2, &[p]),
        None => c,
    }
}

/// Every mode page's current values without block descriptors: the
/// header's device-specific byte carries write protect.
pub fn mode_sense_6(alloc: u8) -> Cdb {
    Cdb::new(6, MODE_SENSE_6).put(1, &[DBD]).put(2, &[ALL_PAGES]).put(4, &[alloc])
}

pub fn read_capacity_10() -> Cdb {
    Cdb::new(10, READ_CAPACITY_10)
}

pub fn read_capacity_16(alloc: u32) -> Cdb {
    Cdb::new(16, SERVICE_ACTION_IN_16).put(1, &[SA_READ_CAPACITY_16]).put(10, &alloc.to_be_bytes())
}

pub fn report_luns(alloc: u32) -> Cdb {
    Cdb::new(12, REPORT_LUNS).put(6, &alloc.to_be_bytes())
}

/// READ or WRITE of `count` blocks at `lba`: the 10-byte form while the
/// range fits its 32-bit address and 16-bit count, the 16-byte form past
/// it. None for no blocks or a range that wraps.
pub fn rw(write: bool, lba: u64, count: u32) -> Option<Cdb> {
    let end = lba.checked_add(count as u64)?;
    if count == 0 {
        return None;
    }
    if end <= 1 << 32 && count <= 0xFFFF {
        let op = if write { WRITE_10 } else { READ_10 };
        return Some(
            Cdb::new(10, op)
                .put(2, &(lba as u32).to_be_bytes())
                .put(7, &(count as u16).to_be_bytes()),
        );
    }
    let op = if write { WRITE_16 } else { READ_16 };
    Some(Cdb::new(16, op).put(2, &lba.to_be_bytes()).put(10, &count.to_be_bytes()))
}

/// The whole cache written back: a zero range names every block. The
/// 16-byte form for a disk past 32-bit addresses.
pub fn synchronize_cache(long: bool) -> Cdb {
    if long {
        Cdb::new(16, SYNCHRONIZE_CACHE_16)
    } else {
        Cdb::new(10, SYNCHRONIZE_CACHE_10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inquiry_standard_and_vpd() {
        assert_eq!(inquiry(None, 96).as_slice(), &[0x12, 0, 0, 0, 96, 0]);
        assert_eq!(inquiry(Some(0x80), 0x100).as_slice(), &[0x12, 1, 0x80, 1, 0, 0]);
    }

    #[test]
    fn read_and_write_take_the_short_form_while_it_reaches() {
        let c = rw(false, 0x1234_5678, 0x80).unwrap();
        assert_eq!(c.as_slice(), &[0x28, 0, 0x12, 0x34, 0x56, 0x78, 0, 0, 0x80, 0]);
        let c = rw(true, 0xFFFF_FFFF, 1).unwrap();
        assert_eq!(c.as_slice()[0], 0x2A, "the last 32-bit block still fits");
    }

    #[test]
    fn read_and_write_take_the_long_form_past_it() {
        let c = rw(false, 0xFFFF_FFFF, 2).unwrap();
        assert_eq!(c.as_slice(), &[0x88, 0, 0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 2, 0, 0]);
        let c = rw(true, 0x1_0000_0000, 1).unwrap();
        assert_eq!(&c.as_slice()[..10], &[0x8A, 0, 0, 0, 0, 1, 0, 0, 0, 0]);
        assert_eq!(rw(false, 0, 0x1_0000).unwrap().as_slice()[0], 0x88, "a count past 16 bits");
    }

    #[test]
    fn refusals() {
        assert!(rw(false, 0, 0).is_none());
        assert!(rw(false, u64::MAX, 1).is_none());
    }

    #[test]
    fn the_rest() {
        assert_eq!(test_unit_ready().as_slice(), &[0; 6]);
        assert_eq!(mode_sense_6(192).as_slice(), &[0x1A, 0x08, 0x3F, 0, 192, 0]);
        assert_eq!(read_capacity_10().as_slice(), &[0x25, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let c = read_capacity_16(32);
        assert_eq!(c.as_slice(), &[0x9E, 0x10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0]);
        assert_eq!(report_luns(0x208).as_slice(), &[0xA0, 0, 0, 0, 0, 0, 0, 0, 2, 8, 0, 0]);
        assert_eq!(synchronize_cache(false).as_slice(), &[0x35, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(synchronize_cache(true).as_slice().len(), 16);
    }
}
