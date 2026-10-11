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

//! The disk layer over a host adapter that answers from simulated units.

use std::collections::{BTreeMap, VecDeque};

use super::*;
use crate::hba::{Outcome, SENSE_MAX};
use crate::sim::Unit;

struct Bus {
    units: BTreeMap<(u16, u16), Unit>,
    faults: VecDeque<Fault>,
    buf: Vec<u8>,
    sent: usize,
}

impl Bus {
    fn new(units: Vec<((u16, u16), Unit)>) -> Self {
        Self {
            units: units.into_iter().collect(),
            faults: VecDeque::new(),
            buf: vec![0; 65536],
            sent: 0,
        }
    }
    fn unit(&mut self, t: u16, l: u16) -> &mut Unit {
        self.units.get_mut(&(t, l)).unwrap()
    }
}

impl Hba for Bus {
    fn buffer(&mut self) -> &mut [u8] {
        &mut self.buf
    }

    fn exec(&mut self, at: Addr, cdb: &[u8], dir: Dir, len: usize) -> Result<Outcome, Fault> {
        self.sent += 1;
        if let Some(f) = self.faults.pop_front() {
            return Err(f);
        }
        let out = if dir == Dir::Out { self.buf[..len].to_vec() } else { vec![] };
        let u = self.units.get_mut(&(at.target, at.lun)).ok_or(Fault::NoTarget)?;
        let a = u.execute(cdb, &out);
        let n = a.data.len().min(len);
        self.buf[..n].copy_from_slice(&a.data[..n]);
        let mut o = Outcome::good();
        o.status = a.status;
        o.sense_len = a.sense.len().min(SENSE_MAX);
        o.sense[..o.sense_len].copy_from_slice(&a.sense[..o.sense_len]);
        o.residual = if dir == Dir::In { (len - n) as u32 } else { 0 };
        Ok(o)
    }
}

const AT: Addr = Addr { target: 0, lun: 0 };

#[test]
fn a_disk_is_found_and_sized() {
    let mut b = Bus::new(vec![((0, 0), Unit::disk(4096, 512))]);
    let d = probe(&mut b, AT).unwrap();
    assert_eq!((d.kind, d.blocks, d.block_bytes), (Kind::Disk, 4096, 512));
    assert!(d.medium && !d.removable && !d.read_only);
    assert_eq!(&d.product[..13], b"QEMU HARDDISK");
    assert_eq!(&d.serial[..d.serial_len], b"drive-scsi0-0-0-0");
    let ops: Vec<u8> = b.unit(0, 0).log.iter().map(|c| c[0]).collect();
    assert_eq!(
        ops,
        vec![0x12, 0x12, 0x00, 0x25, 0x1A],
        "inquiry, serial, ready, capacity, write protect"
    );
}

#[test]
fn a_disk_past_2_tib_is_sized_by_read_capacity_16() {
    let mut b = Bus::new(vec![((0, 0), Unit::disk(1 << 33, 4096))]);
    let d = probe(&mut b, AT).unwrap();
    assert_eq!((d.blocks, d.block_bytes), (1 << 33, 4096));
    assert!(b.unit(0, 0).log.iter().any(|c| c[0] == 0x9E));
}

#[test]
fn a_cd_drive_is_removable_and_read_only_with_or_without_a_disc() {
    let mut b = Bus::new(vec![((1, 0), Unit::cd(1000))]);
    let at = Addr { target: 1, lun: 0 };
    let d = probe(&mut b, at).unwrap();
    assert_eq!(
        (d.kind, d.block_bytes, d.removable, d.read_only, d.medium),
        (Kind::Cd, 2048, true, true, true)
    );
    assert_eq!(d.serial_len, 0, "no serial page");
    assert!(
        !b.unit(1, 0).log.iter().any(|c| c[0] == 0x1A),
        "a CD drive is not asked for write protect"
    );
    b.unit(1, 0).medium = false;
    let mut d = probe(&mut b, at).unwrap();
    assert!(!d.medium);
    assert_eq!(read(&mut b, &d, 0, 1), Err(-123));
    assert_eq!(flush(&mut b, &d), Ok(()));
    b.unit(1, 0).medium = true;
    b.unit(1, 0).attentions = 1;
    assert_eq!(refresh(&mut b, &mut d), Ok(()));
    assert!(d.medium && d.blocks == 1000);
}

#[test]
fn a_fixed_disk_with_no_medium_is_refused() {
    let mut u = Unit::disk(64, 512);
    u.medium = false;
    let mut b = Bus::new(vec![((0, 0), u)]);
    assert_eq!(probe(&mut b, AT).err(), Some(Probe::Refused("not ready")));
}

#[test]
fn a_serial_that_is_not_text_is_left_out() {
    let mut u = Unit::disk(64, 512);
    u.serial = Some(b"SN\x01\x02");
    let mut b = Bus::new(vec![((0, 0), u)]);
    assert_eq!(probe(&mut b, AT).unwrap().serial_len, 0);
}

#[test]
fn nothing_at_the_address_and_other_device_types() {
    let mut b = Bus::new(vec![]);
    assert_eq!(probe(&mut b, AT).err(), Some(Probe::Absent));
    let mut tape = Unit::disk(1, 512);
    tape.device_type = 0x01;
    let mut b = Bus::new(vec![((0, 0), tape)]);
    assert_eq!(probe(&mut b, AT).err(), Some(Probe::Other(1)));
    let mut gone = Unit::disk(1, 512);
    gone.device_type = 0x7F;
    let mut b = Bus::new(vec![((0, 0), gone)]);
    assert_eq!(probe(&mut b, AT).err(), Some(Probe::Absent), "qualifier 3");
}

#[test]
fn a_block_size_the_wire_cannot_carry_is_refused() {
    let mut b = Bus::new(vec![((0, 0), Unit::disk(64, 520))]);
    assert_eq!(probe(&mut b, AT).err(), Some(Probe::Refused("block size not served")));
}

#[test]
fn a_write_protected_disk_is_read_only() {
    let mut u = Unit::disk(64, 512);
    u.write_protect = true;
    let mut b = Bus::new(vec![((0, 0), u)]);
    let d = probe(&mut b, AT).unwrap();
    assert!(d.read_only);
    assert_eq!(write(&mut b, &d, 0, 1), Err(-30));
    assert_eq!(b.unit(0, 0).log.last().unwrap()[0], 0x1A, "refused before it is sent");
}

#[test]
fn reads_and_writes_move_the_blocks() {
    let mut b = Bus::new(vec![((0, 0), Unit::disk(1 << 33, 512))]);
    let d = probe(&mut b, AT).unwrap();
    b.buf[..1024].fill(0xA5);
    assert_eq!(write(&mut b, &d, 7, 2), Ok(()));
    assert_eq!(b.unit(0, 0).block(8), vec![0xA5; 512]);
    b.buf.fill(0);
    assert_eq!(read(&mut b, &d, 7, 2), Ok(1024));
    assert_eq!(&b.buf[..1024], &[0xA5; 1024][..]);
    assert_eq!(b.unit(0, 0).log.last().unwrap()[0], 0x28);
    assert_eq!(read(&mut b, &d, 1 << 32, 1), Ok(512));
    assert_eq!(b.unit(0, 0).log.last().unwrap()[0], 0x88);
}

#[test]
fn a_request_is_checked_before_it_is_sent() {
    let mut b = Bus::new(vec![((0, 0), Unit::disk(64, 512))]);
    let d = probe(&mut b, AT).unwrap();
    let before = b.sent;
    assert_eq!(read(&mut b, &d, 64, 1), Err(-6));
    assert_eq!(read(&mut b, &d, 63, 2), Err(-6));
    assert_eq!(read(&mut b, &d, u64::MAX, 1), Err(-6));
    assert_eq!(read(&mut b, &d, 0, 0), Err(-22));
    assert_eq!(read(&mut b, &d, 0, 129), Err(-22), "more than the buffer");
    assert_eq!(b.sent, before);
}

#[test]
fn unit_attentions_busy_and_resets_are_tried_again() {
    let mut b = Bus::new(vec![((0, 0), Unit::disk(64, 512))]);
    let d = probe(&mut b, AT).unwrap();
    b.unit(0, 0).attentions = 2;
    b.unit(0, 0).busy = 1;
    assert_eq!(read(&mut b, &d, 0, 1), Ok(512));
    b.faults.extend([Fault::Reset, Fault::Busy, Fault::Reset]);
    assert_eq!(read(&mut b, &d, 0, 1), Ok(512), "the fourth try");
    b.faults.extend([Fault::Reset; 4]);
    assert_eq!(read(&mut b, &d, 0, 1), Err(-5), "four tries, then given up");
    b.unit(0, 0).attentions = 4;
    assert_eq!(read(&mut b, &d, 0, 1), Err(-5));
}

#[test]
fn faults_and_sense_end_in_their_status() {
    let mut b = Bus::new(vec![((0, 0), Unit::disk(64, 512))]);
    let d = probe(&mut b, AT).unwrap();
    b.faults.push_back(Fault::Timeout);
    assert_eq!(read(&mut b, &d, 0, 1), Err(-110));
    b.faults.push_back(Fault::Failed);
    assert_eq!(read(&mut b, &d, 0, 1), Err(-5));
    b.unit(0, 0).fail_next = Some((0x3, 0x11, 0x00));
    assert_eq!(read(&mut b, &d, 0, 1), Err(-5));
    b.unit(0, 0).fail_next = Some((0x1, 0x17, 0x01));
    assert_eq!(read(&mut b, &d, 0, 1), Ok(512), "recovered");
    b.unit(0, 0).fail_next = Some((0x7, 0x27, 0x00));
    assert_eq!(write(&mut b, &d, 0, 1), Err(-30));
}

#[test]
fn a_short_data_phase_is_an_error() {
    struct Short(Bus);
    impl Hba for Short {
        fn buffer(&mut self) -> &mut [u8] {
            self.0.buffer()
        }
        fn exec(&mut self, at: Addr, cdb: &[u8], dir: Dir, len: usize) -> Result<Outcome, Fault> {
            let mut o = self.0.exec(at, cdb, dir, len)?;
            if cdb[0] == 0x28 {
                o.residual = 512;
            }
            if cdb[0] == 0x2A {
                o.residual = len as u32 + 1;
            }
            Ok(o)
        }
    }
    let mut b = Short(Bus::new(vec![((0, 0), Unit::disk(64, 512))]));
    let d = probe(&mut b, AT).unwrap();
    assert_eq!(read(&mut b, &d, 0, 2), Err(-5));
    assert_eq!(write(&mut b, &d, 0, 1), Err(-5), "a residual past the length");
}

#[test]
fn flush_sends_synchronize_cache_and_a_unit_without_it_has_nothing_to_flush() {
    let mut b = Bus::new(vec![((0, 0), Unit::disk(64, 512)), ((0, 1), Unit::disk(1 << 33, 512))]);
    let d = probe(&mut b, AT).unwrap();
    assert_eq!(flush(&mut b, &d), Ok(()));
    assert_eq!(b.unit(0, 0).flushes, 1);
    assert_eq!(b.unit(0, 0).log.last().unwrap()[0], 0x35);
    let big = probe(&mut b, Addr { target: 0, lun: 1 }).unwrap();
    assert_eq!(flush(&mut b, &big), Ok(()));
    assert_eq!(b.unit(0, 1).log.last().unwrap()[0], 0x91);
    b.unit(0, 0).sync_supported = false;
    assert_eq!(flush(&mut b, &d), Ok(()));
    b.unit(0, 0).fail_next = Some((0x4, 0x44, 0x00));
    assert_eq!(flush(&mut b, &d), Err(-5));
}

#[test]
fn max_blocks_follows_the_block_size() {
    let mut b = Bus::new(vec![((0, 0), Unit::cd(10))]);
    let d = probe(&mut b, AT).unwrap();
    assert_eq!(max_blocks(&d, 65536), 32);
}
