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

//! A SCSI logical unit as QEMU's scsi-hd and scsi-cd answer one, for the
//! host proofs: this crate's tests and every host driver's proof crate,
//! which mounts this file by path. Never part of a driver.

use std::collections::HashMap;

pub const GOOD: u8 = 0x00;
pub const CHECK: u8 = 0x02;
pub const BUSY: u8 = 0x08;

pub struct Answer {
    pub status: u8,
    pub sense: Vec<u8>,
    pub data: Vec<u8>,
}

pub struct Unit {
    pub device_type: u8,
    pub removable: bool,
    pub blocks: u64,
    pub block_bytes: u32,
    pub store: HashMap<u64, Vec<u8>>,
    /// Unit attentions still to report, one per command.
    pub attentions: u32,
    pub medium: bool,
    pub write_protect: bool,
    pub serial: Option<&'static [u8]>,
    /// What REPORT LUNS lists (LUN 0's answer for the target).
    pub luns: Vec<u16>,
    /// The next command ends in CHECK CONDITION with this key, ASC, ASCQ.
    pub fail_next: Option<(u8, u8, u8)>,
    /// Commands answered BUSY before the next is served.
    pub busy: u32,
    pub sync_supported: bool,
    /// Reads return one block fewer than asked, with GOOD status.
    pub short_read: bool,
    pub flushes: u32,
    pub log: Vec<Vec<u8>>,
    pub vendor: &'static [u8],
    pub product: &'static [u8],
}

impl Unit {
    pub fn disk(blocks: u64, block_bytes: u32) -> Self {
        Self {
            device_type: 0,
            removable: false,
            blocks,
            block_bytes,
            store: HashMap::new(),
            attentions: 0,
            medium: true,
            write_protect: false,
            serial: Some(b"drive-scsi0-0-0-0"),
            luns: vec![0],
            fail_next: None,
            busy: 0,
            sync_supported: true,
            short_read: false,
            flushes: 0,
            log: Vec::new(),
            vendor: b"QEMU",
            product: b"QEMU HARDDISK",
        }
    }

    pub fn cd(blocks: u64) -> Self {
        Self {
            device_type: 5,
            removable: true,
            product: b"QEMU CD-ROM",
            serial: None,
            ..Self::disk(blocks, 2048)
        }
    }

    pub fn block(&self, lba: u64) -> Vec<u8> {
        self.store.get(&lba).cloned().unwrap_or_else(|| vec![0; self.block_bytes as usize])
    }

    pub fn execute(&mut self, cdb: &[u8], out: &[u8]) -> Answer {
        self.log.push(cdb.to_vec());
        if self.busy > 0 {
            self.busy -= 1;
            return Answer { status: BUSY, sense: vec![], data: vec![] };
        }
        let op = cdb[0];
        if self.attentions > 0 && op != 0x12 && op != 0xA0 {
            self.attentions -= 1;
            return check(0x6, 0x29, 0x00);
        }
        if let Some((k, a, q)) = self.fail_next.take() {
            // A recovered error still moves the data; any other moves none.
            if k != 0x1 {
                return check(k, a, q);
            }
            let mut answer = self.serve(cdb, out);
            answer.status = CHECK;
            answer.sense = check(k, a, q).sense;
            return answer;
        }
        self.serve(cdb, out)
    }

    fn serve(&mut self, cdb: &[u8], out: &[u8]) -> Answer {
        let op = cdb[0];
        let be =
            |r: std::ops::Range<usize>| cdb[r].iter().fold(0u64, |acc, &b| acc << 8 | b as u64);
        let good = |data: Vec<u8>| Answer { status: GOOD, sense: vec![], data };
        let needs_medium = matches!(op, 0x00 | 0x25 | 0x9E | 0x28 | 0x88 | 0x2A | 0x8A);
        if needs_medium && !self.medium {
            return check(0x2, 0x3A, 0x00);
        }
        match op {
            0x12 if cdb[1] & 1 != 0 => match (cdb[2], self.serial) {
                (0x80, Some(s)) => {
                    let mut d = vec![self.device_type, 0x80, 0, s.len() as u8];
                    d.extend_from_slice(s);
                    good(cut(d, be(3..5)))
                }
                _ => check(0x5, 0x24, 0x00),
            },
            0x12 => {
                let mut d = vec![b' '; 36];
                d[0] = self.device_type;
                d[1] = if self.removable { 0x80 } else { 0 };
                d[2..8].copy_from_slice(&[5, 2, 31, 0, 0, 0x02]);
                d[8..8 + self.vendor.len()].copy_from_slice(self.vendor);
                d[16..16 + self.product.len()].copy_from_slice(self.product);
                d[32..36].copy_from_slice(b"2.5+");
                good(cut(d, be(3..5)))
            }
            0x00 => good(vec![]),
            0x25 => {
                let last = (self.blocks - 1).min(u32::MAX as u64) as u32;
                let mut d = last.to_be_bytes().to_vec();
                d.extend_from_slice(&self.block_bytes.to_be_bytes());
                good(d)
            }
            0x9E if cdb[1] & 0x1F == 0x10 => {
                let mut d = (self.blocks - 1).to_be_bytes().to_vec();
                d.extend_from_slice(&self.block_bytes.to_be_bytes());
                d.resize(32, 0);
                good(cut(d, be(10..14)))
            }
            0x1A => {
                let wp = if self.write_protect { 0x80 } else { 0 };
                good(cut(vec![3, 0, wp, 0], cdb[4] as u64))
            }
            0x28 | 0x88 | 0x2A | 0x8A => {
                let (lba, count) = if op == 0x28 || op == 0x2A {
                    (be(2..6), be(7..9))
                } else {
                    (be(2..10), be(10..14))
                };
                if lba + count > self.blocks {
                    return check(0x5, 0x21, 0x00);
                }
                let bb = self.block_bytes as usize;
                if op == 0x28 || op == 0x88 {
                    let n = count - u64::from(self.short_read);
                    return good((0..n).flat_map(|i| self.block(lba + i)).collect());
                }
                if self.write_protect {
                    return check(0x7, 0x27, 0x00);
                }
                assert_eq!(out.len(), count as usize * bb, "the write carries its blocks");
                for i in 0..count {
                    let at = i as usize * bb;
                    self.store.insert(lba + i, out[at..at + bb].to_vec());
                }
                good(vec![])
            }
            0x35 | 0x91 if self.sync_supported => {
                self.flushes += 1;
                good(vec![])
            }
            0xA0 => {
                let mut d = ((self.luns.len() * 8) as u32).to_be_bytes().to_vec();
                d.extend_from_slice(&[0; 4]);
                for &l in &self.luns {
                    let e = if l < 256 { [0, l as u8] } else { [0x40 | (l >> 8) as u8, l as u8] };
                    d.extend_from_slice(&[e[0], e[1], 0, 0, 0, 0, 0, 0]);
                }
                good(cut(d, be(6..10)))
            }
            _ => check(0x5, 0x20, 0x00),
        }
    }
}

fn cut(mut d: Vec<u8>, alloc: u64) -> Vec<u8> {
    d.truncate(alloc as usize);
    d
}

/// CHECK CONDITION with fixed format sense data.
pub fn check(key: u8, asc: u8, ascq: u8) -> Answer {
    let mut s = vec![0u8; 18];
    s[0] = 0x70;
    s[2] = key;
    s[7] = 10;
    s[12] = asc;
    s[13] = ascq;
    Answer { status: CHECK, sense: s, data: vec![] }
}
