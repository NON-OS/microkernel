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

//! A model eMMC device, written from JEDEC eMMC 5.1 and not from the
//! driver: its state machine, OCR, CID, CSD, EXT_CSD, SWITCH, busy and
//! storage. The SDHCI model drives it.

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum St {
    Off,
    Idle,
    Ready,
    Ident,
    Stby,
    Tran,
    Data,
    Rcv,
    Prg,
    Inactive,
}

impl St {
    fn code(self) -> u32 {
        match self {
            St::Off | St::Idle | St::Inactive => 0,
            St::Ready => 1,
            St::Ident => 2,
            St::Stby => 3,
            St::Tran => 4,
            St::Data => 5,
            St::Rcv => 6,
            St::Prg => 7,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CardCfg {
    /// Voltage window and access mode, busy bit clear.
    pub ocr: u32,
    /// CMD1 polls with a window before the busy bit is set.
    pub ready_after: u32,
    pub sec_count: u32,
    pub spec_vers: u8,
    pub c_size: u32,
    pub c_size_mult: u8,
    pub read_bl_len: u8,
    pub device_type: u8,
    pub partition_config: u8,
    pub cache_kib: u32,
    pub cache_on: bool,
    pub refuse_hs: bool,
    /// Data on eight lines arrives corrupted.
    pub broken8: bool,
    pub prg_ms: u64,
    /// After a write the card never leaves programming state.
    pub stuck_after_write: bool,
    /// No device: nothing answers.
    pub absent: bool,
    pub name: [u8; 6],
    pub mid: u8,
    pub prv: u8,
    pub psn: u32,
}

impl Default for CardCfg {
    fn default() -> Self {
        Self {
            ocr: 0x40ff_8080,
            ready_after: 3,
            sec_count: 61_071_360,
            spec_vers: 4,
            c_size: 0xfff,
            c_size_mult: 7,
            read_bl_len: 9,
            device_type: 0x57,
            partition_config: 0,
            cache_kib: 0,
            cache_on: false,
            refuse_hs: false,
            broken8: false,
            prg_ms: 3,
            stuck_after_write: false,
            absent: false,
            name: *b"DF4032",
            mid: 0x45,
            prv: 0x10,
            psn: 0x1234_5678,
        }
    }
}

pub enum Reply {
    /// No response expected (CMD0).
    Nothing,
    /// The command was not answered.
    Silent,
    Short(u32),
    Long(u128),
}

pub enum Phase {
    None,
    Read(Vec<u8>),
    Write { lba: u64 },
}

pub struct Out {
    pub reply: Reply,
    pub phase: Phase,
    /// The command signals busy for this long after its response.
    pub busy_ms: u64,
}

impl Out {
    fn reply(r: Reply) -> Self {
        Self { reply: r, phase: Phase::None, busy_ms: 0 }
    }
}

pub struct Card {
    pub cfg: CardCfg,
    pub state: St,
    pub rca: u16,
    pub polls: u32,
    pub width: u8,
    pub hs: bool,
    pub ext: [u8; 512],
    pub store: HashMap<u64, [u8; 512]>,
    pub busy_until: u64,
    pub switch_error: bool,
    pub count: Option<u16>,
    pub flushes: u32,
    pub cmd1_args: Vec<u32>,
    pub writes: u32,
}

pub const OUT_OF_RANGE: u32 = 1 << 31;
pub const SWITCH_ERROR: u32 = 1 << 7;
pub const READY_FOR_DATA: u32 = 1 << 8;
pub const BUSY: u32 = 1 << 31;

impl Card {
    pub fn new(cfg: CardCfg) -> Self {
        let mut ext = [0u8; 512];
        ext[192] = 8;
        ext[194] = 2;
        ext[196] = cfg.device_type;
        ext[160] = 0x07;
        ext[179] = cfg.partition_config;
        ext[199] = 3;
        ext[212..216].copy_from_slice(&cfg.sec_count.to_le_bytes());
        ext[217] = 0x11;
        ext[221] = 0x10;
        ext[223] = 0x02;
        ext[224] = 0x01;
        ext[226] = 32;
        ext[168] = 32;
        ext[229] = 0x02;
        ext[230] = 0x02;
        ext[231] = 0x55;
        ext[232] = 0x02;
        ext[248] = 10;
        ext[249..253].copy_from_slice(&cfg.cache_kib.to_le_bytes());
        ext[33] = cfg.cache_on as u8;
        Self {
            cfg,
            state: St::Off,
            rca: 0,
            polls: 0,
            width: 1,
            hs: false,
            ext,
            store: HashMap::new(),
            busy_until: 0,
            switch_error: false,
            count: None,
            flushes: 0,
            cmd1_args: Vec::new(),
            writes: 0,
        }
    }

    pub fn power(&mut self, on: bool) {
        if on && self.state == St::Off {
            self.state = St::Idle;
            self.polls = 0;
            self.width = 1;
            self.hs = false;
            self.ext[183] = 0;
            self.ext[185] = 0;
            self.count = None;
        } else if !on {
            self.state = St::Off;
        }
    }

    pub fn cid(&self) -> u128 {
        let c = &self.cfg;
        let mut v: u128 = (c.mid as u128) << 120 | 1u128 << 112 | 0x01u128 << 104;
        for (i, &b) in c.name.iter().enumerate() {
            v |= (b as u128) << (96 - 8 * i as u32);
        }
        v |= (c.prv as u128) << 48 | (c.psn as u128) << 16 | 0x51u128 << 8 | 0x2bu128 << 1 | 1;
        v
    }

    pub fn csd(&self) -> u128 {
        let c = &self.cfg;
        3u128 << 126
            | (c.spec_vers as u128) << 122
            | 0x32u128 << 96
            | (c.read_bl_len as u128) << 80
            | (c.c_size as u128) << 62
            | (c.c_size_mult as u128) << 47
            | 1
    }

    pub fn sector_mode(&self) -> bool {
        self.cfg.ocr & (3 << 29) == 2 << 29
    }

    pub fn capacity(&self) -> u64 {
        if self.sector_mode() {
            self.cfg.sec_count as u64
        } else {
            let c = &self.cfg;
            ((c.c_size as u64 + 1) << (c.c_size_mult + 2) << c.read_bl_len) / 512
        }
    }

    pub fn tick(&mut self, now: u64) {
        if self.state == St::Prg
            && now >= self.busy_until
            && !(self.cfg.stuck_after_write && self.writes > 0)
        {
            self.state = St::Tran;
        }
    }

    pub fn status(&self) -> u32 {
        let mut s = self.state.code() << 9;
        if self.state == St::Tran {
            s |= READY_FOR_DATA;
        }
        if self.switch_error {
            s |= SWITCH_ERROR;
        }
        s
    }

    fn lba(&self, arg: u32) -> Option<u64> {
        if self.sector_mode() {
            Some(arg as u64)
        } else if arg.is_multiple_of(512) {
            Some(arg as u64 / 512)
        } else {
            None
        }
    }

    pub fn block(&self, lba: u64) -> [u8; 512] {
        self.store.get(&lba).copied().unwrap_or_else(|| {
            let mut b = [0u8; 512];
            for (i, x) in b.iter_mut().enumerate() {
                *x = (lba as u8).wrapping_mul(31).wrapping_add(i as u8);
            }
            b
        })
    }

    /// One command, with the host's Block Count register for an open-ended
    /// multi-block read.
    pub fn command(&mut self, idx: u8, arg: u32, host_blocks: u16, now: u64) -> Out {
        self.tick(now);
        if self.cfg.absent || self.state == St::Off || self.state == St::Inactive {
            return Out::reply(Reply::Silent);
        }
        let before = self.status();
        match idx {
            0 => {
                self.state = St::Idle;
                self.polls = 0;
                Out::reply(Reply::Nothing)
            }
            1 if matches!(self.state, St::Idle | St::Ready) => {
                self.cmd1_args.push(arg);
                let window = arg & 0x00ff_ff80;
                if window == 0 {
                    return Out::reply(Reply::Short(self.cfg.ocr));
                }
                if window & self.cfg.ocr == 0 {
                    self.state = St::Inactive;
                    return Out::reply(Reply::Silent);
                }
                self.polls += 1;
                if self.polls >= self.cfg.ready_after {
                    self.state = St::Ready;
                    Out::reply(Reply::Short(self.cfg.ocr | BUSY))
                } else {
                    Out::reply(Reply::Short(self.cfg.ocr))
                }
            }
            2 if self.state == St::Ready => {
                self.state = St::Ident;
                Out::reply(Reply::Long(self.cid()))
            }
            3 if self.state == St::Ident => {
                self.rca = (arg >> 16) as u16;
                self.state = St::Stby;
                Out::reply(Reply::Short(before))
            }
            9 if self.state == St::Stby && (arg >> 16) as u16 == self.rca => {
                Out::reply(Reply::Long(self.csd()))
            }
            7 if (arg >> 16) as u16 == self.rca && self.state == St::Stby => {
                self.state = St::Tran;
                Out::reply(Reply::Short(before))
            }
            13 if (arg >> 16) as u16 == self.rca && self.state != St::Idle => {
                let s = self.status();
                self.switch_error = false;
                Out::reply(Reply::Short(s))
            }
            8 if self.state == St::Tran && self.cfg.spec_vers >= 4 => {
                self.state = St::Data;
                Out {
                    reply: Reply::Short(before),
                    phase: Phase::Read(self.ext.to_vec()),
                    busy_ms: 0,
                }
            }
            6 if self.state == St::Tran && self.cfg.spec_vers >= 4 => {
                let access = (arg >> 24) & 3;
                let index = ((arg >> 16) & 0xff) as usize;
                let value = ((arg >> 8) & 0xff) as u8;
                let ok = access == 3
                    && match index {
                        179 => true,
                        183 => value <= 2,
                        185 => value <= 1 && !self.cfg.refuse_hs,
                        32 => value == 1,
                        _ => false,
                    };
                if ok {
                    match index {
                        183 => self.width = [1, 4, 8][value as usize],
                        185 => self.hs = value == 1,
                        32 => self.flushes += 1,
                        _ => {}
                    }
                    if index != 32 {
                        self.ext[index] = value;
                    }
                } else {
                    self.switch_error = true;
                }
                self.state = St::Prg;
                self.busy_until = now + self.cfg.prg_ms;
                Out { reply: Reply::Short(before), phase: Phase::None, busy_ms: self.cfg.prg_ms }
            }
            16 if self.state == St::Tran => Out::reply(Reply::Short(before)),
            23 if self.state == St::Tran => {
                self.count = Some((arg & 0xffff) as u16);
                Out::reply(Reply::Short(before))
            }
            17 | 18 | 24 | 25 if self.state == St::Tran => {
                let n = if idx == 17 || idx == 24 {
                    1
                } else {
                    self.count.unwrap_or(host_blocks) as u64
                };
                let Some(lba) = self.lba(arg) else {
                    return Out::reply(Reply::Short(before | 1 << 30));
                };
                if lba + n > self.capacity() {
                    return Out::reply(Reply::Short(before | OUT_OF_RANGE));
                }
                if idx == 17 || idx == 18 {
                    self.state = St::Data;
                    let mut v = Vec::new();
                    for i in 0..n {
                        v.extend_from_slice(&self.block(lba + i));
                    }
                    Out { reply: Reply::Short(before), phase: Phase::Read(v), busy_ms: 0 }
                } else {
                    self.state = St::Rcv;
                    Out { reply: Reply::Short(before), phase: Phase::Write { lba }, busy_ms: 0 }
                }
            }
            12 if matches!(self.state, St::Data | St::Rcv) => {
                self.end(now);
                Out { reply: Reply::Short(before), phase: Phase::None, busy_ms: 0 }
            }
            _ => Out::reply(Reply::Silent),
        }
    }

    /// The data phase is over (all blocks moved, or CMD12).
    pub fn end(&mut self, now: u64) {
        self.count = None;
        match self.state {
            St::Data => self.state = St::Tran,
            St::Rcv => {
                self.state = St::Prg;
                self.busy_until = now + self.cfg.prg_ms;
            }
            _ => {}
        }
    }

    pub fn take_write(&mut self, lba: u64, data: &[u8], now: u64) {
        for (i, chunk) in data.chunks(512).enumerate() {
            let mut b = [0u8; 512];
            b.copy_from_slice(chunk);
            self.store.insert(lba + i as u64, b);
        }
        self.writes += 1;
        self.end(now);
    }
}
