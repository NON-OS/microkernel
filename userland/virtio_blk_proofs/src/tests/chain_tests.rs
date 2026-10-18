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

//! The chain one request is, as the device reads it: the 16-byte header at
//! the header grant, the data (device-writable for a read only), the status
//! byte the device writes, linked 0, 1, 2 from head 0; the available entry
//! naming head 0, its index published last.

use nonos_devmodel::FakeBar;

use super::model::{Rig, DATA_PHYS, HEADER_PHYS, IRQ};
use super::script::answer_each;
use crate::constants::{STATUS_OFFSET, VIRTIO_BLK_T_FLUSH, VIRTIO_BLK_T_IN, VIRTIO_BLK_T_OUT};
use crate::io::submit;
use crate::queue::{avail_offset, Direction};

const NEXT: u16 = 1;
const WRITE: u16 = 2;

/// Descriptor `i` as (address, length, flags, next).
fn desc(ring: &FakeBar, i: usize) -> (u64, u32, u16, u16) {
    let at = 16 * i;
    let addr = u64::from(ring.wrote32(at)) | u64::from(ring.wrote32(at + 4)) << 32;
    (addr, ring.wrote32(at + 8), ring.wrote16(at + 12), ring.wrote16(at + 14))
}

fn posted(dir: Direction, lba: u64, n: u32, answer: u32) -> Rig {
    let rig = Rig::modern();
    let mut q = rig.queue();
    answer_each(&rig, answer);
    assert_eq!(submit(rig.transport(), &mut q, IRQ, dir, lba, n), Ok(()));
    rig
}

#[test]
fn a_read_chains_header_data_for_the_device_and_status() {
    let rig = posted(Direction::Read, 0x1_0000_0002, 2, 1025);
    let h = &rig.header;
    assert_eq!((h.wrote32(0), h.wrote32(4)), (VIRTIO_BLK_T_IN, 0));
    assert_eq!((h.wrote32(8), h.wrote32(12)), (2, 1), "the sector, little endian");
    let status = HEADER_PHYS + STATUS_OFFSET as u64;
    assert_eq!(desc(&rig.ring, 0), (HEADER_PHYS, 16, NEXT, 1));
    assert_eq!(desc(&rig.ring, 1), (DATA_PHYS, 1024, NEXT | WRITE, 2));
    assert_eq!(desc(&rig.ring, 2), (status, 1, WRITE, 0));
    let avail = avail_offset(super::model::SIZE);
    let entry = (rig.ring.wrote16(avail), rig.ring.wrote16(avail + 4));
    assert_eq!((entry, rig.ring.wrote16(avail + 2)), ((0, 0), 1), "flags, head 0, index 1");
}

#[test]
fn a_write_gives_the_device_its_data_to_read_and_a_flush_has_none() {
    let rig = posted(Direction::Write, 7, 64, 1);
    assert_eq!(rig.header.wrote32(0), VIRTIO_BLK_T_OUT);
    assert_eq!(desc(&rig.ring, 1), (DATA_PHYS, 64 * 512, NEXT, 2));
    let rig = posted(Direction::Flush, 7, 0, 1);
    assert_eq!(rig.header.wrote32(0), VIRTIO_BLK_T_FLUSH);
    let status = HEADER_PHYS + STATUS_OFFSET as u64;
    assert_eq!(desc(&rig.ring, 0), (HEADER_PHYS, 16, NEXT, 1));
    assert_eq!(desc(&rig.ring, 1), (status, 1, WRITE, 0), "status straight after the header");
}
