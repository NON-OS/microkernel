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

//! READ and WRITE through the engine: the command the disk sees, the table
//! the engine walks, the sectors that move.

use super::util::{ask, body, pattern, qemu, request, rw, status, up};
use crate::model::{table, Drive};
use crate::wire::{OP_READ, OP_WRITE};

#[test]
fn a_read_returns_the_sectors_the_disk_holds() {
    let (mut d, p) = up(qemu(4096));
    for lba in 5..8 {
        p.borrow_mut().ch[0].drives[0].as_mut().unwrap().store.insert(lba, pattern(lba));
    }
    let r = ask(&mut d, &request(OP_READ, 0, 41, &rw(5, 3, &[])));
    assert_eq!(status(&r), 0);
    let want: Vec<u8> = (5..8).flat_map(pattern).collect();
    assert_eq!(body(&r), &want[..]);
    let p = p.borrow();
    assert_eq!(p.ch[0].log.last(), Some(&(0xC8, 0, 5, 3)), "READ DMA, LBA28");
    // One region, exactly the bytes moved, end of table.
    assert_eq!(table(p.ch[0].prdt), vec![(0x0010_1000, 1536)]);
    assert_eq!(p.ch[0].bmic & 1, 0, "the engine is stopped after");
    assert_eq!(p.ch[0].bmis & 0x06, 0, "and its latched status cleared");
}

#[test]
fn a_write_lands_on_the_disk_and_reads_back() {
    let (mut d, p) = up(qemu(4096));
    let data: Vec<u8> = (100..102).flat_map(pattern).collect();
    let r = ask(&mut d, &request(OP_WRITE, 0, 42, &rw(100, 2, &data)));
    assert_eq!(status(&r), 0);
    assert_eq!(p.borrow().ch[0].log.last(), Some(&(0xCA, 0, 100, 2)));
    assert_eq!(p.borrow().ch[0].drives[0].as_ref().unwrap().sector(101), pattern(101));
    let r = ask(&mut d, &request(OP_READ, 0, 43, &rw(100, 2, &[])));
    assert_eq!(body(&r), &data[..]);
}

#[test]
fn a_full_transfer_is_cut_at_the_64_kib_boundary() {
    let (mut d, p) = up(qemu(4096));
    for lba in 0..128 {
        p.borrow_mut().ch[0].drives[0].as_mut().unwrap().store.insert(lba, pattern(lba));
    }
    let r = ask(&mut d, &request(OP_READ, 0, 44, &rw(0, 128, &[])));
    assert_eq!(status(&r), 0);
    assert_eq!(body(&r).len(), 65536);
    assert_eq!(body(&r)[512 * 127..], pattern(127));
    // The buffer starts at 0x101000; the boundary at 0x110000 splits it.
    assert_eq!(table(p.borrow().ch[0].prdt), vec![(0x0010_1000, 0xF000), (0x0011_0000, 0x1000)]);
    assert_eq!(p.borrow().ch[0].log.last(), Some(&(0xC8, 0, 0, 128)));
}

#[test]
fn a_range_past_lba28_takes_the_ext_command_with_every_byte() {
    let big = (1u64 << 32) + 0x100;
    let (mut d, p) = up(qemu(big));
    let lba = (1u64 << 32) + 0x12;
    p.borrow_mut().ch[0].drives[0].as_mut().unwrap().store.insert(lba, pattern(lba));
    let r = ask(&mut d, &request(OP_READ, 0, 45, &rw(lba, 1, &[])));
    assert_eq!(status(&r), 0);
    assert_eq!(body(&r), &pattern(lba)[..]);
    assert_eq!(p.borrow().ch[0].log.last(), Some(&(0x25, 0, lba, 1)));
}

#[test]
fn a_range_straddling_2_to_the_28_takes_lba48() {
    let (mut d, p) = up(qemu(1 << 29));
    let data = [7u8; 1024];
    let lba = (1u64 << 28) - 1;
    let r = ask(&mut d, &request(OP_WRITE, 0, 46, &rw(lba, 2, &data)));
    assert_eq!(status(&r), 0);
    assert_eq!(p.borrow().ch[0].log.last(), Some(&(0x35, 0, lba, 2)));
    let r = ask(&mut d, &request(OP_WRITE, 0, 47, &rw(lba - 1, 1, &data[..512])));
    assert_eq!(status(&r), 0);
    assert_eq!(p.borrow().ch[0].log.last(), Some(&(0xCA, 0, lba - 1, 1)), "below it, LBA28");
}

#[test]
fn the_slave_on_the_secondary_is_unit_3() {
    let slots = [[None, None], [Some(Drive::disk(64, true)), Some(Drive::disk(64, true))]];
    let (mut d, p) = up(slots);
    p.borrow_mut().ch[1].drives[1].as_mut().unwrap().store.insert(9, pattern(9));
    let r = ask(&mut d, &request(OP_READ, 3, 48, &rw(9, 1, &[])));
    assert_eq!(body(&r), &pattern(9)[..]);
    let p = p.borrow();
    assert_eq!(p.ch[1].log.last(), Some(&(0xC8, 1, 9, 1)), "drive 1 selected");
    assert_eq!(p.ch[1].prdt, 0x0010_0000, "the secondary's engine was pointed at the table");
}

#[test]
fn a_transfer_that_takes_its_time_is_waited_for() {
    let (mut d, p) = up(qemu(64));
    p.borrow_mut().ch[0].delay = 200;
    p.borrow_mut().ch[0].drives[0].as_mut().unwrap().store.insert(1, pattern(1));
    let r = ask(&mut d, &request(OP_READ, 0, 49, &rw(1, 1, &[])));
    assert_eq!(status(&r), 0);
    assert_eq!(body(&r), &pattern(1)[..]);
}
