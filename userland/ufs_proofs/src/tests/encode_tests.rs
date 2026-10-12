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

//! The words the driver builds, against JESD223 and JESD220 field by field:
//! the UTRD, the PRDT entry, the UPIUs and the CDBs.

use crate::ufs::facts::upiu::*;
use crate::ufs::scsi::cdb::{report_luns, rw, sync_cache, test_unit_ready};
use crate::ufs::slot::descriptor::{descriptor, Dir};
use crate::ufs::slot::prdt::entry;
use crate::ufs::upiu::request::{command, flag_query, nop_out};

#[test]
fn utrd_fields() {
    let d = descriptor(1, Dir::FromDevice, 0x1_2345_6780, 1);
    let dw = |i: usize| u32::from_le_bytes(d[i * 4..i * 4 + 4].try_into().unwrap());
    assert_eq!(dw(0), 0x1000_0000 | 0x0400_0000, "CT 1, device to host, no interrupt");
    assert_eq!(dw(2), 0x0F, "OCS invalid until the host writes it");
    assert_eq!((dw(4), dw(5)), (0x2345_6780, 0x1));
    assert_eq!(dw(6), 128 << 16 | 128, "response at 512 bytes, 512 long, in dwords");
    assert_eq!(dw(7), 256 << 16 | 1, "PRDT at 1024 bytes, one entry");
    assert_eq!(descriptor(0, Dir::ToDevice, 0, 0)[3], 0x02, "host to device is DD 1");
}

#[test]
fn prdt_entries_are_dword_granular_and_bounded() {
    let e = entry(0xABCD_0000, 4096).unwrap();
    assert_eq!(u64::from_le_bytes(e[0..8].try_into().unwrap()), 0xABCD_0000);
    assert_eq!(u32::from_le_bytes(e[12..16].try_into().unwrap()), 4095);
    assert!(entry(0, 1 << 18).is_some());
    for (bus, len) in [(0, 0), (0, 6), (2, 8), (0, (1 << 18) + 4)] {
        assert!(entry(bus, len).is_none(), "{bus:#x} {len}");
    }
}

#[test]
fn upiu_vectors() {
    assert_eq!(nop_out(), [0u8; 32]);
    let c = command(3, FLAG_READ, 0x1000, &[0x28, 0, 0, 0, 0, 9, 0, 0, 1, 0]);
    assert_eq!(&c[..4], &[COMMAND, 0x40, 3, 0]);
    assert_eq!(&c[12..16], &[0, 0, 0x10, 0], "expected length big-endian");
    assert_eq!(&c[16..26], &[0x28, 0, 0, 0, 0, 9, 0, 0, 1, 0]);
    let q = flag_query(QUERY_WRITE, OP_SET_FLAG, FLAG_DEVICE_INIT);
    assert_eq!((q[0], q[5], q[12], q[13]), (0x16, 0x81, 0x06, 0x01));
}

#[test]
fn cdb_vectors() {
    let (b, n) = rw(false, 0x0102_0304, 0x10);
    assert_eq!(&b[..n], &[0x28, 0, 1, 2, 3, 4, 0, 0, 0x10, 0]);
    let (b, n) = rw(true, 0x1_0000_0000, 1);
    assert_eq!(n, 16);
    assert_eq!(&b[..14], &[0x8A, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1]);
    assert_eq!(rw(false, 0, 0x1_0000).1, 16, "past a 16-bit count");
    assert_eq!(test_unit_ready().1, 6);
    assert_eq!(&sync_cache().0[..10], &[0x35, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let (b, n) = report_luns(72);
    assert_eq!(&b[..n], &[0xA0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0]);
}
