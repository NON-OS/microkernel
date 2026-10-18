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

//! One IDE channel as the ATA task file and the PIIX bus master engine
//! answer it (ACS-3; 82371AB data sheet 2.7; QEMU's hw/ide core and piix):
//! the registers the host writes, with LBA48's high-byte history, the
//! selected drive's status, IDENTIFY's data phase, the soft reset, and the
//! engine walking the descriptor table through the DMA grant.

use super::drive::{Drive, Fault, Kind};
use nonos_libc::dma_at;

pub const ST_ERR: u8 = 0x01;
pub const ST_DRQ: u8 = 0x08;
pub const ST_READY: u8 = 0x50;
pub const ST_BSY: u8 = 0x80;
pub const ABORT: u8 = ST_READY | ST_ERR;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pending {
    pub to_memory: bool,
    pub drive: u8,
    pub lba: u64,
    pub count: u32,
}

#[derive(Default)]
pub struct Chan {
    pub drives: [Option<Drive>; 2],
    pub sel: u8,
    /// (previous, current) for each register LBA48 writes twice.
    pub count: (u8, u8),
    pub lo: (u8, u8),
    pub mid: (u8, u8),
    pub hi: (u8, u8),
    pub dev: u8,
    pub devctl: u8,
    pub status: [u8; 2],
    pub hung: [bool; 2],
    pub data: Vec<u16>,
    pub pending: Option<Pending>,
    pub bmic: u8,
    pub bmis: u8,
    pub prdt: u32,
    /// The channel's devices never leave busy after a reset.
    pub busy_forever: bool,
    /// Engine status reads before a started transfer completes.
    pub delay: u32,
    /// Engine status reads, after the disk is done, before the engine goes
    /// inactive: its FIFO still emptying into memory.
    pub drain: u32,
    pub lagging: u32,
    /// Every command executed: (opcode, drive, lba, count).
    pub log: Vec<(u8, u8, u64, u32)>,
    pub resets: u32,
}

impl Chan {
    fn floating(&self) -> bool {
        self.drives.iter().all(Option::is_none)
    }

    fn present(&self) -> bool {
        self.drives[self.sel as usize].is_some()
    }

    pub fn status(&mut self) -> u8 {
        if self.floating() {
            return 0xFF;
        }
        if !self.present() {
            return 0;
        }
        self.status[self.sel as usize]
    }

    pub fn cmd_read(&mut self, reg: u16, width: u8) -> u32 {
        if self.floating() {
            return if width == 2 { 0xFFFF } else { 0xFF };
        }
        match reg {
            0 => {
                assert_eq!(width, 2, "data is read a word at a time");
                assert!(self.status() & ST_DRQ != 0, "data read with DRQ clear");
                let w = self.data.remove(0);
                if self.data.is_empty() {
                    self.status[self.sel as usize] = ST_READY;
                }
                u32::from(w)
            }
            2 => self.count.1.into(),
            3 => self.lo.1.into(),
            4 => self.mid.1.into(),
            5 => self.hi.1.into(),
            6 => self.dev.into(),
            7 => self.status().into(),
            _ => panic!("read of command block register {reg}"),
        }
    }

    pub fn cmd_write(&mut self, reg: u16, v: u8) {
        let shift = |r: &mut (u8, u8)| *r = (r.1, v);
        match reg {
            2 => shift(&mut self.count),
            3 => shift(&mut self.lo),
            4 => shift(&mut self.mid),
            5 => shift(&mut self.hi),
            6 => {
                self.dev = v;
                self.sel = (v >> 4) & 1;
            }
            7 => self.exec(v),
            _ => panic!("write of command block register {reg}"),
        }
    }

    pub fn control(&mut self, v: u8) {
        let was = self.devctl;
        self.devctl = v;
        if v & 0x04 != 0 && was & 0x04 == 0 {
            self.status = [ST_BSY; 2];
        }
        if v & 0x04 == 0 && was & 0x04 != 0 {
            self.resets += 1;
            self.sel = 0;
            self.hung = [false; 2];
            self.pending = None;
            for (i, d) in self.drives.iter().enumerate() {
                self.status[i] = match d {
                    _ if self.busy_forever => ST_BSY,
                    Some(d) if d.kind == Kind::Packet => 0,
                    _ => ST_READY,
                };
            }
            let sig = match &self.drives[0] {
                Some(d) if d.kind == Kind::Packet => (0x14, 0xEB),
                _ => (0, 0),
            };
            self.mid.1 = sig.0;
            self.hi.1 = sig.1;
        }
    }

    fn exec(&mut self, op: u8) {
        let s = self.sel as usize;
        let Some(d) = self.drives[s].as_mut() else { return };
        assert!(self.status[s] & (ST_BSY | ST_DRQ) == 0, "a command written to a busy drive");
        if d.fault == Fault::Hang {
            self.hung[s] = true;
            self.status[s] = ST_BSY;
            return;
        }
        let ext = matches!(op, 0x25 | 0x35 | 0xEA);
        if d.kind == Kind::Packet || (ext && !d.lba48) {
            if d.kind == Kind::Packet {
                self.mid.1 = 0x14;
                self.hi.1 = 0xEB;
            }
            self.status[s] = ABORT;
            return;
        }
        let (lba, count) = if ext {
            let lba = [self.hi.0, self.mid.0, self.lo.0, self.hi.1, self.mid.1, self.lo.1]
                .iter()
                .fold(0u64, |acc, &b| acc << 8 | u64::from(b));
            let count = u32::from(self.count.0) << 8 | u32::from(self.count.1);
            (lba, if count == 0 { 65536 } else { count })
        } else {
            let lba = u64::from(self.dev & 0x0F) << 24
                | u64::from(self.hi.1) << 16
                | u64::from(self.mid.1) << 8
                | u64::from(self.lo.1);
            let count = u32::from(self.count.1);
            (lba, if count == 0 { 256 } else { count })
        };
        self.log.push((op, self.sel, lba, count));
        match op {
            0xEC => {
                self.data = d.identify.to_vec();
                self.status[s] = ST_READY | ST_DRQ;
            }
            0xC8 | 0x25 | 0xCA | 0x35 => {
                assert!(self.dev & 0x40 != 0, "a DMA command without LBA addressing");
                if lba + count as u64 > d.sectors {
                    self.status[s] = ABORT;
                    return;
                }
                let to_memory = matches!(op, 0xC8 | 0x25);
                self.pending = Some(Pending { to_memory, drive: self.sel, lba, count });
                self.status[s] = ST_BSY | ST_READY;
                if self.bmic & 1 != 0 {
                    self.maybe_complete();
                }
            }
            0xE7 | 0xEA => {
                d.flushes += 1;
                self.status[s] = ST_READY;
            }
            _ => self.status[s] = ABORT,
        }
    }

    pub fn bm_read(&mut self, reg: u16, width: u8) -> u32 {
        match (reg, width) {
            (0, 1) => self.bmic.into(),
            (2, 1) => {
                if self.lagging > 0 {
                    self.lagging -= 1;
                    if self.lagging == 0 {
                        self.bmis &= !0x01;
                    }
                }
                if self.pending.is_some() && self.bmic & 1 != 0 {
                    self.maybe_complete();
                }
                self.bmis.into()
            }
            (4, 4) => self.prdt,
            _ => panic!("bus master read {reg}/{width}"),
        }
    }

    pub fn bm_write(&mut self, reg: u16, width: u8, v: u32) {
        match (reg, width) {
            (0, 1) => {
                let was = self.bmic;
                self.bmic = v as u8 & 0x09;
                if v & 1 != 0 && was & 1 == 0 {
                    self.bmis |= 0x01;
                    if self.pending.is_some() {
                        self.maybe_complete();
                    }
                } else if v & 1 == 0 {
                    // Clearing start halts the engine and clears active.
                    self.bmis &= !0x01;
                }
            }
            (2, 1) => {
                let v = v as u8;
                self.bmis = (self.bmis & 0x07 & !(v & 0x06)) | (v & 0x60);
            }
            (4, 4) => {
                assert_eq!(v & 3, 0, "the table is four-byte aligned");
                self.prdt = v;
            }
            _ => panic!("bus master write {reg}/{width}"),
        }
    }

    fn maybe_complete(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
            return;
        }
        let Some(p) = self.pending.take() else { return };
        assert_eq!(self.bmic & 0x08 != 0, p.to_memory, "the engine's direction is the command's");
        let s = p.drive as usize;
        let d = self.drives[s].as_mut().expect("the drive");
        if d.fault == Fault::DmaError {
            d.fault = Fault::None;
            self.status[s] = ABORT;
            return;
        }
        let regions = table(self.prdt);
        let total: usize = regions.iter().map(|r| r.1).sum();
        let short = d.fault == Fault::Short;
        if short {
            d.fault = Fault::None;
        }
        let need = (p.count as usize - usize::from(short)) * 512;
        assert!(total >= need, "a table shorter than the transfer");
        let mut bytes = vec![0u8; need];
        if p.to_memory {
            for i in 0..(need / 512) as u64 {
                let at = i as usize * 512;
                bytes[at..at + 512].copy_from_slice(&d.sector(p.lba + i));
            }
        }
        let mut at = 0usize;
        for &(addr, len) in &regions {
            let n = len.min(need - at);
            let host = dma_at(addr as u64);
            // SAFETY: dma_at checked the region's start lies in a grant; the
            // grant spans the buffer the driver named.
            unsafe {
                if p.to_memory {
                    std::ptr::copy_nonoverlapping(bytes[at..].as_ptr(), host, n);
                } else {
                    std::ptr::copy_nonoverlapping(host, bytes[at..].as_mut_ptr(), n);
                }
            }
            at += n;
        }
        if !p.to_memory {
            for i in 0..(need / 512) as u64 {
                let at = i as usize * 512;
                d.store.insert(p.lba + i, bytes[at..at + 512].try_into().unwrap());
            }
        }
        self.status[s] = ST_READY;
        if d.fault == Fault::ErrorAtEnd {
            d.fault = Fault::None;
            self.status[s] = ABORT;
        }
        self.bmis |= 0x04;
        // The engine stays active while the table names more than moved,
        // and for its drain after a whole one.
        if total == need {
            if self.drain == 0 {
                self.bmis &= !0x01;
            } else {
                self.lagging = self.drain;
            }
        }
    }
}

/// The table at `prdt`: (address, bytes) per region up to end of table.
pub fn table(prdt: u32) -> Vec<(u32, usize)> {
    let mut out = Vec::new();
    for i in 0.. {
        let p = dma_at(prdt as u64 + i * 8);
        // SAFETY: a descriptor is eight bytes inside the grant's first page.
        let e = unsafe { std::slice::from_raw_parts(p, 8) };
        let addr = u32::from_le_bytes(e[0..4].try_into().unwrap());
        let count = u16::from_le_bytes(e[4..6].try_into().unwrap());
        let flags = u16::from_le_bytes(e[6..8].try_into().unwrap());
        let len = if count == 0 { 65536 } else { count as usize };
        assert_eq!(addr & 1, 0, "a region starts on an even address");
        assert_eq!(
            addr as u64 >> 16,
            (addr as u64 + len as u64 - 1) >> 16,
            "a region crosses 64 KiB"
        );
        out.push((addr, len));
        if flags & 0x8000 != 0 {
            return out;
        }
        assert!(i < 16, "no end of table");
    }
    unreachable!()
}
