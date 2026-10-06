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

//! Reads, writes and flushes through the real disk surface, against the
//! model: the data lands where it was asked, each request is made of the
//! commands it should be, and nothing outside the disk or the buffer is
//! ever asked of the card.

use crate::emmc::disk::{MAX_SECTORS, SECTOR_SIZE};
use crate::emmc::error::EmmcError;
use crate::model::{CardCfg, HostCfg, Rig, TestDisk};

fn up(card: CardCfg) -> (Rig, TestDisk) {
    let rig = Rig::new(HostCfg::default(), card);
    let disk = rig.disk(true).expect("bring-up");
    (rig, disk)
}

fn pattern(lba: u64, n: u32, salt: u8) -> Vec<u8> {
    (0..n as usize * SECTOR_SIZE).map(|i| (i as u8).wrapping_mul(13) ^ (lba as u8) ^ salt).collect()
}

#[test]
fn writes_read_back_at_every_size() {
    let (rig, mut disk) = up(CardCfg::default());
    for (lba, n) in
        [(0u64, 1u32), (1, 2), (100, 7), (4096, 64), (61_071_360 - 64, 64), (61_071_359, 1)]
    {
        let data = pattern(lba, n, 0x5c);
        disk.write(lba, n, &data).expect("write");
        let mut back = vec![0u8; n as usize * SECTOR_SIZE];
        disk.read(lba, n, &mut back).expect("read");
        assert_eq!(back, data, "lba {lba} n {n}");
        let sim = rig.sim.borrow();
        for i in 0..n as u64 {
            let off = i as usize * SECTOR_SIZE;
            assert_eq!(&sim.card.store[&(lba + i)][..], &data[off..off + SECTOR_SIZE]);
        }
    }
    rig.assert_clean();
}

#[test]
fn reads_return_what_the_card_holds() {
    let (rig, mut disk) = up(CardCfg::default());
    let mut out = vec![0u8; 3 * SECTOR_SIZE];
    disk.read(77, 3, &mut out).expect("read");
    let sim = rig.sim.borrow();
    for i in 0..3u64 {
        assert_eq!(&out[i as usize * SECTOR_SIZE..][..SECTOR_SIZE], &sim.card.block(77 + i)[..]);
    }
}

#[test]
fn one_sector_is_a_single_block_command_and_more_go_after_cmd23() {
    let (rig, mut disk) = up(CardCfg::default());
    let start = rig.cmds().len();
    let mut buf = vec![0u8; 64 * SECTOR_SIZE];
    disk.read(5, 1, &mut buf).unwrap();
    disk.read(5, 9, &mut buf).unwrap();
    disk.write(5, 1, &buf[..SECTOR_SIZE]).unwrap();
    disk.write(5, 64, &buf).unwrap();
    rig.assert_clean();
    let c: Vec<_> =
        rig.cmds()[start..].iter().map(|c| (c.index, c.arg, c.mode, c.blocks)).collect();
    let mut it = c.iter().filter(|x| x.0 != 13);
    assert_eq!(it.next(), Some(&(17, 5, 0x13, 1)));
    assert_eq!(it.next().map(|x| (x.0, x.1)), Some((23, 9)));
    assert_eq!(it.next(), Some(&(18, 5, 0x33, 9)));
    assert_eq!(it.next(), Some(&(24, 5, 0x03, 1)));
    assert_eq!(it.next().map(|x| (x.0, x.1)), Some((23, 64)));
    assert_eq!(it.next(), Some(&(25, 5, 0x23, 64)));
    // Every write is followed by a status read before the reply.
    let after24 = c.iter().position(|x| x.0 == 24).unwrap();
    assert_eq!(c[after24 + 1].0, 13);
    assert_eq!(c.last().unwrap().0, 13);
}

#[test]
fn a_full_request_is_one_32_kib_descriptor_with_end() {
    let (rig, mut disk) = up(CardCfg::default());
    let mut buf = vec![0u8; 64 * SECTOR_SIZE];
    disk.read(0, 64, &mut buf).unwrap();
    let sim = rig.sim.borrow();
    let t = sim.tables.last().unwrap();
    assert_eq!(t.len(), 1);
    assert_eq!((t[0].attr, t[0].len, t[0].addr), (0x23, 32768, crate::model::DATA_BUS));
}

#[test]
fn requests_outside_the_disk_or_the_buffer_send_nothing() {
    let (rig, mut disk) = up(CardCfg::default());
    let before = rig.cmds().len();
    let cap = disk.capacity_sectors();
    let mut buf = vec![0u8; 65 * SECTOR_SIZE];
    assert_eq!(disk.read(cap, 1, &mut buf), Err(EmmcError::OutOfRange));
    assert_eq!(disk.read(cap - 1, 2, &mut buf), Err(EmmcError::OutOfRange));
    assert_eq!(disk.read(u64::MAX, 1, &mut buf), Err(EmmcError::OutOfRange));
    assert_eq!(disk.read(0, 0, &mut buf), Err(EmmcError::OutOfRange));
    assert_eq!(disk.read(0, MAX_SECTORS + 1, &mut buf), Err(EmmcError::OutOfRange));
    assert_eq!(disk.read(0, 2, &mut buf[..SECTOR_SIZE]), Err(EmmcError::OutOfRange));
    assert_eq!(disk.write(0, 2, &buf[..SECTOR_SIZE]), Err(EmmcError::OutOfRange));
    assert_eq!(disk.write(cap, 1, &buf[..SECTOR_SIZE]), Err(EmmcError::OutOfRange));
    assert_eq!(rig.cmds().len(), before);
}

#[test]
fn flush_switches_flush_cache_when_the_cache_is_on() {
    let (rig, mut disk) = up(CardCfg { cache_kib: 512, cache_on: true, ..CardCfg::default() });
    assert!(disk.card.cache_on());
    disk.flush().expect("flush");
    rig.assert_clean();
    assert_eq!(rig.sim.borrow().card.flushes, 1);
    let six = rig.cmds().iter().rev().find(|c| c.index == 6).unwrap().arg;
    assert_eq!(six, 0x0320_0100);
}

#[test]
fn flush_is_nothing_to_do_when_the_cache_is_off() {
    for card in [
        CardCfg { cache_kib: 512, cache_on: false, ..CardCfg::default() },
        CardCfg { cache_kib: 0, cache_on: true, ..CardCfg::default() },
    ] {
        let (rig, mut disk) = up(card);
        let before = rig.cmds().len();
        disk.flush().expect("flush");
        assert_eq!(rig.cmds().len(), before);
    }
}

#[test]
fn a_card_without_cmd23_gets_auto_cmd12_and_byte_addresses() {
    let card = CardCfg { ocr: 0x00ff_8080, spec_vers: 2, c_size: 0x7ff, ..CardCfg::default() };
    let (rig, mut disk) = up(card);
    assert!(!disk.card.cmd23 && !disk.card.sector_mode);
    let data = pattern(10, 4, 1);
    disk.write(10, 4, &data).unwrap();
    let mut back = vec![0u8; 4 * SECTOR_SIZE];
    disk.read(10, 4, &mut back).unwrap();
    assert_eq!(back, data);
    rig.assert_clean();
    let c = rig.cmds();
    let w = c.iter().find(|c| c.index == 25).unwrap();
    assert_eq!((w.arg, w.mode), (10 * 512, 0x27));
    let r = c.iter().find(|c| c.index == 18).unwrap();
    assert_eq!((r.arg, r.mode), (10 * 512, 0x37));
    assert!(!c.iter().any(|c| c.index == 23));
}

#[test]
fn info_payloads_describe_the_card() {
    let (_rig, mut disk) = up(CardCfg::default());
    assert_eq!(disk.capacity_sectors(), 61_071_360);
    let n = disk.names();
    assert_eq!(n.model(), b"SanDisk DF4032");
    assert_eq!(&n.serial, b"12345678");
    let c = disk.controller_info();
    assert_eq!(u32::from_le_bytes(c[0..4].try_into().unwrap()), crate::model::GLK_CAPS);
    assert_eq!(c[20], 1);
    let p = disk.port_entry();
    assert_eq!(&p[0..4], &[0, 1, 1, 5]);
    // CMD: the card's status, in transfer state and ready for data.
    assert_eq!(u32::from_le_bytes(p[16..20].try_into().unwrap()), 4 << 9 | 1 << 8);
}
