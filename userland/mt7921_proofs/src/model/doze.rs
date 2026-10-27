// NONOS Operating System (AGPL-3.0-or-later)

//! The card taken back by its firmware, as a laptop's link power management
//! or a firmware reset can leave it: `OWN_SYNC` reads set, and until the
//! driver clears its ownership again every other register reads zero and
//! every write to one is lost. A deep doze also loses the WFDMA's state:
//! its re-init flag (`MT_WFDMA_NEED_REINIT`), its switches and the six
//! rings' registers. And a card a warm reboot left mid-transfer: the
//! WFDMA's busy bits stay set until its logic reset is pulsed.

use super::chip::{Chip, LPCTL};

/// `MT_WFDMA_DUMMY_CR` (0x54000120) in BAR0, and `MT_WFDMA0_GLO_CFG`.
pub const DUMMY_CR: u32 = 0x2120;
pub const GLO_CFG: u32 = 0xd4208;
const RST: u32 = 0xd4100;
const RINGS: [u32; 6] = [0xd4300, 0xd4400, 0xd4410, 0xd4500, 0xd4520, 0xd4540];

impl Chip {
    /// The firmware takes the card; with `lose`, the WFDMA forgets.
    pub fn doze(&mut self, lose: bool) {
        (self.own_sync, self.faults.doze) = (true, true);
        if lose {
            for r in
                RINGS.iter().flat_map(|r| [*r, r + 4, r + 8, r + 12]).chain([DUMMY_CR, GLO_CFG])
            {
                self.regs.insert(r, 0);
            }
        }
    }

    /// Whether `off` is out of reach: the firmware holds the card.
    pub(super) fn dozing(&self, off: u32) -> bool {
        self.faults.doze && self.own_sync && off != LPCTL
    }

    /// The busy bits a stuck WFDMA adds to `GLO_CFG`.
    pub(super) fn busy(&self, off: u32) -> u32 {
        if off == GLO_CFG && self.faults.stuck_busy {
            0xa
        } else {
            0
        }
    }

    /// What a write of `v` to `off` stores: the busy bits are read-only,
    /// and the logic reset pulsed (`MT_WFDMA0_RST_LOGIC_RST`) frees them.
    pub(super) fn stored(&mut self, off: u32, v: u32) -> u32 {
        if off == RST && v & 0x10 != 0 {
            self.faults.stuck_busy = false;
        }
        if off == GLO_CFG {
            v & !0xa
        } else {
            v
        }
    }
}
