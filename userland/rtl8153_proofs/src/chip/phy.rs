// NONOS Operating System (AGPL-3.0-or-later)
//! The PHY's registers behind the window, and writes to the register file.

use super::{Access, Chip, Stuck, PLA};

impl Chip {
    pub(super) fn phy_get(&mut self, addr: u16) -> u16 {
        self.log.push(Access::PhyRead(addr));
        let v = self.phy.get(&addr).copied().unwrap_or(0);
        match addr {
            0xa420 if self.stuck == Stuck::PhySettle => 1,
            0xa420 if v == 5 && self.left.contains_key("phy_on") => {
                let state = if self.ready("phy_on", Stuck::PhyOn) { 3 } else { 5 };
                self.phy.insert(addr, state);
                state
            }
            0xa400 if v & 0x8000 != 0 && self.ready("phy_reset", Stuck::PhyReset) => {
                self.phy.insert(addr, v & !0x8000);
                v & !0x8000
            }
            _ => v,
        }
    }

    fn phy_put(&mut self, addr: u16, v: u16) {
        self.log.push(Access::PhyWrite(addr, v));
        let was = self.phy.get(&addr).copied().unwrap_or(0);
        self.phy.insert(addr, v);
        match addr {
            0xa400 if was & 0x0800 != 0 && v & 0x0800 == 0 => self.start("phy_on"),
            0xa400 if v & 0x8000 != 0 => self.start("phy_reset"),
            0xa430 if v & 0x0004 == 0 => self.start("aldps"),
            _ => {}
        }
    }

    pub(super) fn write_dword(&mut self, block: u16, at: u16, enable: u16, data: u32) {
        if block == PLA && (0xb000..=0xbffc).contains(&at) {
            for (lanes, shift) in [(0x3, 0), (0xc, 2)] {
                if enable & lanes == lanes {
                    let addr = self.page | ((at + shift) & 0x0fff);
                    self.phy_put(addr, (data >> (8 * shift)) as u16);
                }
            }
            return;
        }
        self.log.push(Access::Write(block, at, enable, data));
        let mut v = self.stored(block, at);
        for lane in 0..4 {
            if enable & (1 << lane) != 0 {
                let m = 0xffu32 << (8 * lane);
                v = (v & !m) | (data & m);
            }
        }
        self.dwords.insert((block, at), v);
        match (block, at) {
            (PLA, 0xe86c) => self.page = v as u16,
            (PLA, 0xe810) if v & 0x1000_0000 != 0 => self.start("reset"),
            (PLA, 0xe8dc) if enable & 0xc != 0 => self.start("list"),
            _ => {}
        }
    }
}
