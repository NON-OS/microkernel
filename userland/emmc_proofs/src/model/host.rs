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

//! A model SD Host Controller, written from the SD Host Controller
//! Simplified Specification 3.00 and not from the driver: register file,
//! resets, power, clock, the command engine with its response checks, ADMA2
//! walking real host memory, interrupt status, and the rules a driver must
//! keep (clock limits per card state, the response flags each command
//! needs, transfer mode and block registers matching the command, no Nop
//! descriptor with End). A broken rule is recorded in `violations`.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::card::{Card, CardCfg, Phase, Reply, St};

#[derive(Debug, Clone)]
pub struct HostCfg {
    pub caps: u32,
    pub caps1: u32,
    pub version: u16,
    /// Writes of SD Bus Power on that do not stick.
    pub power_ignores: u32,
    /// Reads of Software Reset that still show the bits set.
    pub reset_reads: u32,
    /// Card detect reads present.
    pub cd_inserted: bool,
}

/// Intel Gemini Lake eMMC-like: spec 3.00, 200 MHz, ADMA2, 64-bit,
/// high speed, 1.8 V only, embedded slot, 8-bit not advertised.
pub const GLK_CAPS: u32 = (200 << 8) | 1 << 19 | 1 << 21 | 1 << 26 | 1 << 28 | 1 << 30;

impl Default for HostCfg {
    fn default() -> Self {
        Self {
            caps: GLK_CAPS,
            caps1: 0x0000_0807,
            version: 0x1002,
            power_ignores: 0,
            reset_reads: 2,
            cd_inserted: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmdRec {
    pub index: u8,
    pub arg: u32,
    pub word: u16,
    pub mode: u16,
    pub blocks: u16,
    pub blksz: u16,
    pub hz: u32,
    pub width: u8,
    pub hs: bool,
    pub at: u64,
}

/// One data phase, as the registers and the card set it up.
struct Xfer {
    index: u8,
    mode: u16,
    blocks: u16,
    blksz: u16,
    card_reads: bool,
    bytes: Vec<u8>,
    lba: u64,
}

pub struct Desc {
    pub attr: u16,
    pub len: u32,
    pub addr: u64,
}

pub struct Sim {
    pub cfg: HostCfg,
    regs: [u8; 256],
    int: u32,
    pub card: Card,
    now: Rc<Cell<u64>>,
    /// DMA memory: (bus address, host pointer, length).
    pub mem: Vec<(u64, usize, usize)>,
    tc_at: Option<u64>,
    srst_left: u32,
    power_ignores: u32,
    pub cmds: Vec<CmdRec>,
    pub violations: Vec<String>,
    /// Voltage select of every SD Bus Power on that stuck, in order.
    pub power_ons: Vec<u8>,
    pub fail_next_data: u32,
    pub tables: Vec<Vec<Desc>>,
}

/// The register-level view the driver's `Mmio` sees.
#[derive(Clone)]
pub struct SimBus(pub Rc<RefCell<Sim>>);

const RESP_FLAGS: &[(u8, u16)] = &[
    (0, 0x00),
    (1, 0x02),
    (2, 0x09),
    (3, 0x1a),
    (6, 0x1b),
    (7, 0x1a),
    (8, 0x3a),
    (9, 0x09),
    (12, 0xdb),
    (13, 0x1a),
    (16, 0x1a),
    (17, 0x3a),
    (18, 0x3a),
    (23, 0x1a),
    (24, 0x3a),
    (25, 0x3a),
];

impl Sim {
    pub fn new(cfg: HostCfg, card: CardCfg, now: Rc<Cell<u64>>) -> Self {
        let mut s = Self {
            power_ignores: cfg.power_ignores,
            cfg,
            regs: [0; 256],
            int: 0,
            card: Card::new(card),
            now,
            mem: Vec::new(),
            tc_at: None,
            srst_left: 0,
            cmds: Vec::new(),
            violations: Vec::new(),
            power_ons: Vec::new(),
            fail_next_data: 0,
            tables: Vec::new(),
        };
        s.reset_all();
        // Firmware left the host running at some setting.
        s.regs[0x28] = 0x3e;
        s.regs[0x29] = 0x0b;
        s
    }

    fn now(&self) -> u64 {
        self.now.get()
    }

    fn reset_all(&mut self) {
        self.regs = [0; 256];
        self.regs[0x40..0x44].copy_from_slice(&self.cfg.caps.to_le_bytes());
        self.regs[0x44..0x48].copy_from_slice(&self.cfg.caps1.to_le_bytes());
        self.regs[0xfe..0x100].copy_from_slice(&self.cfg.version.to_le_bytes());
        self.int = 0;
        self.tc_at = None;
        self.card.power(false);
    }

    fn get(&self, off: u32, n: usize) -> u32 {
        let mut v = 0u32;
        for i in 0..n {
            v |= (self.regs[off as usize + i] as u32) << (8 * i);
        }
        v
    }

    fn put(&mut self, off: u32, n: usize, v: u32) {
        for i in 0..n {
            self.regs[off as usize + i] = (v >> (8 * i)) as u8;
        }
    }

    pub fn host_width(&self) -> u8 {
        let hc = self.regs[0x28];
        if hc & 0x20 != 0 {
            8
        } else if hc & 0x02 != 0 {
            4
        } else {
            1
        }
    }

    /// The SD clock in Hz, 0 when off.
    pub fn sd_hz(&self) -> u32 {
        let clk = self.get(0x2c, 2);
        if clk & 0x4 == 0 || clk & 0x1 == 0 {
            return 0;
        }
        let base = ((self.cfg.caps >> 8) & 0xff) * 1_000_000;
        let n = ((clk >> 8) & 0xff) | ((clk >> 6) & 0x3) << 8;
        if (self.cfg.version & 0xff) >= 2 {
            if n == 0 {
                base
            } else {
                base / (2 * n)
            }
        } else {
            let n = (clk >> 8) & 0xff;
            if n == 0 {
                base
            } else {
                base / (2 * n)
            }
        }
    }

    fn present(&mut self) -> u32 {
        let mut ps = 1 << 17 | 0xf << 20 | 1 << 24;
        if self.cfg.cd_inserted || self.regs[0x28] & 0xc0 == 0xc0 {
            ps |= 1 << 16 | 1 << 18;
        }
        if self.tc_at.is_some() {
            ps |= 1 << 1 | 1 << 2;
        }
        ps
    }

    fn poll_tc(&mut self) {
        if let Some(t) = self.tc_at {
            if self.now() >= t {
                self.tc_at = None;
                self.int |= 1 << 1;
            }
        }
    }

    pub fn read(&mut self, off: u32, n: usize) -> u32 {
        self.poll_tc();
        match off {
            0x24 => self.present() & mask(n),
            0x2f => {
                if self.srst_left > 0 {
                    self.srst_left -= 1;
                    self.regs[0x2f] as u32
                } else {
                    self.regs[0x2f] = 0;
                    0
                }
            }
            0x30 => self.int & mask(n),
            0x32 => (self.int >> 16) & mask(n),
            0x2c => {
                let mut c = self.get(0x2c, 2);
                if c & 1 != 0 {
                    c |= 2;
                }
                c & mask(n)
            }
            _ => self.get(off, n),
        }
    }

    pub fn write(&mut self, off: u32, n: usize, v: u32) {
        self.poll_tc();
        match (off, n) {
            (0x2f, _) => {
                let v = v as u8;
                if v & 1 != 0 {
                    self.reset_all();
                } else if v & 6 != 0 {
                    self.int = 0;
                    self.tc_at = None;
                }
                self.regs[0x2f] = v;
                self.srst_left = self.cfg.reset_reads;
            }
            (0x29, _) => {
                let mut v = v as u8;
                if v & 1 != 0 && self.regs[0x29] & 1 == 0 && self.power_ignores > 0 {
                    self.power_ignores -= 1;
                    v &= !1;
                }
                if v & 1 != 0 && self.regs[0x29] & 1 == 0 {
                    let sel = (v >> 1) & 7;
                    if !matches!(sel, 5..=7) {
                        self.violations.push(format!("bus power on with voltage select {sel}"));
                    }
                    if sel == 5 && self.cfg.caps & 1 << 26 == 0
                        || sel == 6 && self.cfg.caps & 1 << 25 == 0
                        || sel == 7 && self.cfg.caps & 1 << 24 == 0
                    {
                        self.violations
                            .push(format!("voltage select {sel} the host does not offer"));
                    }
                    self.power_ons.push(sel);
                    self.card.power(true);
                }
                if v & 1 == 0 {
                    self.card.power(false);
                }
                self.regs[0x29] = v;
            }
            (0x30, 4) => self.int &= !v,
            (0x30, 2) => self.int &= !(v & 0xffff),
            (0x32, 2) => self.int &= !((v & 0xffff) << 16),
            (0x0c, 4) => {
                self.put(0x0c, 2, v & 0xffff);
                self.command((v >> 16) as u16);
            }
            (0x0e, 2) => self.command(v as u16),
            _ => self.put(off, n, v),
        }
    }

    fn violate(&mut self, s: String) {
        self.violations.push(s);
    }

    fn command(&mut self, word: u16) {
        self.put(0x0e, 2, word as u32);
        let index = ((word >> 8) & 0x3f) as u8;
        let arg = self.get(0x08, 4);
        let mode = self.get(0x0c, 2) as u16;
        let blocks = self.get(0x06, 2) as u16;
        let blksz = self.get(0x04, 2) as u16;
        let now = self.now();
        self.cmds.push(CmdRec {
            index,
            arg,
            word,
            mode,
            blocks,
            blksz,
            hz: self.sd_hz(),
            width: self.host_width(),
            hs: self.regs[0x28] & 0x04 != 0,
            at: now,
        });
        let ps = self.present();
        let abort = word >> 6 & 3 == 3;
        if ps & 1 != 0 || (ps & 2 != 0 && (word & 0x20 != 0 || word & 3 == 3) && !abort) {
            self.violate(format!("CMD{index} issued while inhibited"));
        }
        match RESP_FLAGS.iter().find(|(i, _)| *i == index) {
            Some((_, f)) if *f == word & 0xff => {}
            Some((_, f)) => {
                self.violate(format!("CMD{index} flags {:#x}, want {:#x}", word & 0xff, f))
            }
            None => self.violate(format!("CMD{index} is not a command the driver should send")),
        }
        let hz = self.sd_hz();
        if self.regs[0x29] & 1 == 0 || hz == 0 {
            self.int |= 1 << 15 | 1 << 16;
            return;
        }
        let limit = match self.card.state {
            St::Idle | St::Ready | St::Ident | St::Off | St::Inactive => 400_000,
            _ if self.card.hs => 52_000_000,
            _ => 26_000_000,
        };
        if hz > limit {
            self.violate(format!("CMD{index} at {hz} Hz in {:?}, limit {limit}", self.card.state));
        }
        let data = word & 0x20 != 0;
        let out = self.card.command(index, arg, blocks, now);
        match out.reply {
            // A command that expects no response completes whether or not
            // anything is listening.
            Reply::Silent if word & 3 == 0 => {}
            Reply::Silent => {
                self.int |= 1 << 15 | 1 << 16;
                return;
            }
            Reply::Nothing => {}
            Reply::Short(r) => self.put(0x10, 4, r),
            Reply::Long(v) => {
                let reg = v >> 8;
                for i in 0..4 {
                    self.put(0x10 + 4 * i, 4, (reg >> (32 * i)) as u32);
                }
            }
        }
        self.int |= 1;
        match out.phase {
            Phase::None => {
                if data {
                    // The card sends nothing: the host times out the data.
                    self.int |= 1 << 15 | 1 << 20;
                } else if word & 3 == 3 {
                    self.tc_at = Some(now + out.busy_ms);
                    self.poll_tc();
                }
            }
            Phase::Read(bytes) => {
                self.data(Xfer { index, mode, blocks, blksz, card_reads: true, bytes, lba: 0 })
            }
            Phase::Write { lba } => self.data(Xfer {
                index,
                mode,
                blocks,
                blksz,
                card_reads: false,
                bytes: Vec::new(),
                lba,
            }),
        }
    }

    fn data(&mut self, x: Xfer) {
        let Xfer { index, mode, blocks, blksz, card_reads, bytes, lba } = x;
        let now = self.now();
        if !matches!(index, 8 | 17 | 18 | 24 | 25) {
            self.violate(format!("CMD{index} sent with data"));
        }
        let multi = index == 18 || index == 25;
        let had_count = self.card.count.is_some() || !multi;
        let want_mode = 1
            | 2
            | if card_reads { 0x10 } else { 0 }
            | if multi { 0x20 } else { 0 }
            | if multi && !had_count { 0x04 } else { 0 };
        if mode != want_mode {
            self.violate(format!("CMD{index} transfer mode {mode:#x}, want {want_mode:#x}"));
        }
        if blksz & 0xfff != 512 {
            self.violate(format!("CMD{index} block size {blksz:#x}"));
        }
        let n = blocks as usize;
        if !multi && n != 1 {
            self.violate(format!("CMD{index} single block with count {n}"));
        }
        if let Some(c) = self.card.count {
            if c as usize != n {
                self.violate(format!("CMD23 count {c}, block count {n}"));
            }
        }
        let len = n * 512;
        if card_reads && bytes.len() != len {
            self.violate(format!("CMD{index} reads {} bytes, host expects {len}", bytes.len()));
        }
        let segs = match self.walk(len) {
            Some(s) => s,
            None => {
                self.int |= 1 << 15 | 1 << 25;
                return;
            }
        };
        if self.host_width() != self.card.width || self.fail_next_data > 0 {
            if self.fail_next_data > 0 {
                self.fail_next_data -= 1;
            }
            // The card sends or takes the blocks; the host sees CRC errors.
            self.card.end(now);
            self.int |= 1 << 15 | 1 << 21;
            return;
        }
        if card_reads {
            let mut bytes = bytes;
            if self.card.cfg.broken8 && self.host_width() == 8 {
                for b in bytes.iter_mut().step_by(7) {
                    *b ^= 0x5a;
                }
            }
            let mut o = 0;
            for (addr, l) in segs {
                let p = self.ptr(addr, l);
                unsafe { std::ptr::copy_nonoverlapping(bytes[o..].as_ptr(), p, l) };
                o += l;
            }
            self.card.end(now);
        } else {
            let mut v = Vec::with_capacity(len);
            for (addr, l) in segs {
                let p = self.ptr(addr, l);
                v.extend_from_slice(unsafe { std::slice::from_raw_parts(p, l) });
            }
            self.card.take_write(lba, &v, now);
        }
        if mode & 0x04 != 0 {
            // Auto CMD12.
            let _ = self.card.command(12, 0, 0, now);
        }
        if card_reads {
            self.int |= 1 << 1;
        } else {
            self.tc_at = Some(now + 1);
        }
    }

    fn ptr(&mut self, addr: u64, len: usize) -> *mut u8 {
        for &(bus, p, l) in &self.mem {
            if addr >= bus && addr + len as u64 <= bus + l as u64 {
                return (p + (addr - bus) as usize) as *mut u8;
            }
        }
        panic!("DMA to {addr:#x}+{len:#x} outside every region");
    }

    /// Walk the ADMA2 table the registers name; the data segments, or None
    /// on an ADMA error.
    fn walk(&mut self, len: usize) -> Option<Vec<(u64, usize)>> {
        let sel = (self.regs[0x28] >> 3) & 3;
        let wide = match sel {
            2 => false,
            3 => true,
            _ => {
                self.violate(format!("data command with DMA select {sel}"));
                return None;
            }
        };
        if wide && self.cfg.caps & 1 << 28 == 0 {
            self.violate("64-bit ADMA2 on a host without it".into());
        }
        let mut at = self.get(0x58, 4) as u64;
        if wide {
            at |= (self.get(0x5c, 4) as u64) << 32;
        }
        let dl = if wide { 12 } else { 8 };
        if !at.is_multiple_of(if wide { 8 } else { 4 }) {
            self.violate(format!("descriptor table at {at:#x} misaligned"));
        }
        let mut segs = Vec::new();
        let mut table = Vec::new();
        let mut total = 0usize;
        for _ in 0..64 {
            let p = self.ptr(at, dl);
            let d = unsafe { std::slice::from_raw_parts(p, dl) };
            let attr = u16::from_le_bytes([d[0], d[1]]);
            let l = u16::from_le_bytes([d[2], d[3]]) as u32;
            let addr = if wide {
                u64::from_le_bytes(d[4..12].try_into().unwrap())
            } else {
                u32::from_le_bytes(d[4..8].try_into().unwrap()) as u64
            };
            table.push(Desc { attr, len: l, addr });
            if attr & 1 == 0 {
                self.tables.push(table);
                return None;
            }
            let act = (attr >> 4) & 3;
            let end = attr & 2 != 0;
            match act {
                2 => {
                    let l = if l == 0 { 65536 } else { l as usize };
                    if addr % 4 != 0 {
                        self.violate(format!("data address {addr:#x} misaligned"));
                    }
                    if !wide && addr + l as u64 > 1 << 32 {
                        self.violate("32-bit descriptor past 4 GiB".into());
                    }
                    segs.push((addr, l));
                    total += l;
                }
                0 => {
                    if end {
                        self.violate("Nop descriptor with End".into());
                    }
                }
                3 => {
                    at = addr;
                    continue;
                }
                _ => self.violate("reserved Act".into()),
            }
            if end {
                self.tables.push(table);
                if total != len {
                    self.violate(format!("descriptors move {total} bytes, command moves {len}"));
                    return None;
                }
                return Some(segs);
            }
            at += dl as u64;
        }
        self.violate("descriptor table without End".into());
        self.tables.push(table);
        None
    }
}

fn mask(n: usize) -> u32 {
    if n >= 4 {
        u32::MAX
    } else {
        (1u32 << (8 * n)) - 1
    }
}

impl crate::emmc::env::Mmio for SimBus {
    fn r8(&self, off: u32) -> u8 {
        self.0.borrow_mut().read(off, 1) as u8
    }
    fn r16(&self, off: u32) -> u16 {
        self.0.borrow_mut().read(off, 2) as u16
    }
    fn r32(&self, off: u32) -> u32 {
        self.0.borrow_mut().read(off, 4)
    }
    fn w8(&self, off: u32, v: u8) {
        self.0.borrow_mut().write(off, 1, v as u32)
    }
    fn w16(&self, off: u32, v: u16) {
        self.0.borrow_mut().write(off, 2, v as u32)
    }
    fn w32(&self, off: u32, v: u32) {
        self.0.borrow_mut().write(off, 4, v)
    }
}
