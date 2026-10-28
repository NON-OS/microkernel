// NONOS Operating System (AGPL-3.0-or-later)
//! The register file's dwords, and the bits the chip sets itself.

use super::{Access, Chip, Stuck, PLA};

const WINDOW: std::ops::RangeInclusive<u16> = 0xb000..=0xbffc;

impl Chip {
    /// A readiness: not while stuck, else after its countdown.
    pub(super) fn ready(&mut self, name: &'static str, kind: Stuck) -> bool {
        let n = self.left.entry(name).or_insert(0);
        if self.stuck == kind {
            return false;
        }
        *n = n.saturating_sub(1);
        *n == 0
    }

    pub(super) fn start(&mut self, name: &'static str) {
        self.left.insert(name, 3);
    }

    pub fn stored(&self, block: u16, at: u16) -> u32 {
        let version = ((self.code | 0x830f) as u32) << 16; // noise outside the field
        let initial = if (block, at) == (PLA, 0xe610) { version } else { 0 };
        *self.dwords.get(&(block, at)).unwrap_or(&initial)
    }

    pub(super) fn read_dword(&mut self, block: u16, at: u16, lanes: u16) -> u32 {
        if block == PLA && WINDOW.contains(&at) {
            let high = lanes == 0xcc;
            let addr = self.page | ((at + 2 * high as u16) & 0x0fff);
            let v = self.phy_get(addr) as u32;
            return if high { v << 16 } else { v };
        }
        self.log.push(Access::Read(block, at));
        let mut v = self.stored(block, at);
        let flip = |v: &mut u32, bits: u32, on: bool| *v = if on { *v | bits } else { *v & !bits };
        match (block, at) {
            (PLA, 0xe004) => flip(&mut v, 0x0002, self.ready("autoload", Stuck::Autoload)),
            (PLA, 0xe000) => {
                let off = self.phy.get(&0xa430).copied().unwrap_or(0) & 0x0004 == 0;
                flip(&mut v, 0x0100, off && self.ready("aldps", Stuck::Aldps));
            }
            (PLA, 0xe810) if v & 0x1000_0000 != 0 && self.ready("reset", Stuck::MacReset) => {
                v &= !0x1c00_0000; // reset done; receive and transmit off
                self.dwords.insert((block, at), v);
            }
            (PLA, 0xe84c) => {
                flip(&mut v, 0x0200_0000, self.ready("list", Stuck::LinkList));
                flip(&mut v, 0x3000_0000, self.stuck != Stuck::Drain);
            }
            (PLA, 0xe610) => flip(&mut v, 0x0800, self.stuck != Stuck::Drain),
            (PLA, 0xe908) => v = (v & 0xffff_0000) | self.link,
            _ => {}
        }
        v
    }
}
