// NONOS Operating System (AGPL-3.0-or-later)

//! Read the board facts the PHY needs out of the efuse. The RF front-end option
//! and the package type are burned per module and are not autoloaded into any
//! MAC register, so the PHY cannot be configured without them: the wrong RF
//! front-end selects the wrong radio tables. This reimplements the two-step
//! rtw88 read: dump the physical efuse a byte at a time through the efuse control
//! register, then de-shuffle its packed block/word format into the logical map
//! the option byte is indexed from. The 8821c defines no efuse-access grant op,
//! so only the bank select and the 2.5V LDO disable bracket the read. Values are
//! rtw88 facts (`reg.h`, `efuse.c`, `rtw8821c.c`), reimplemented not copied.

use core::sync::atomic::Ordering;

use super::dump::{LAST_ADDR, LAST_CTL};
use super::layout::*;
use super::{deshuffle, dump_physical};
use crate::regs::Mmio;

/// The board facts the PHY bring-up needs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EfuseInfo {
    /// The cut (chip revision) version from the config register.
    pub cut: u8,
    /// The RF front-end option, five bits, selecting the radio tables.
    pub rfe: u8,
    /// The package type, one bit alongside the RF front-end option.
    pub pkg: u8,
    /// The station MAC address burned into the efuse. The chip does NOT autoload
    /// it into the MAC-ID registers on this board, so the driver must read it here
    /// and program the registers itself; otherwise the station runs on an all-zero
    /// MAC, which associates but leaves the access point unable to address the
    /// unicast EAPOL handshake, so the four-way never completes.
    pub mac: [u8; 6],
}

/// Read the efuse and the cut version and extract the PHY board facts, or `None`
/// if the efuse never completed a byte read.
pub fn read<M: Mmio>(mmio: &M) -> Option<EfuseInfo> {
    let phys = dump_physical(mmio)?;
    // A window that reads back all-ones sets the ready flag on every read, so
    // every byte "completes" instantly and the poll never stalls: an unreadable
    // efuse is otherwise indistinguishable from a blank one. Programming the RF
    // from that map picks a front-end the chip does not have.
    if phys.iter().all(|&b| b == 0xFF) {
        LAST_CTL.store(0xFFFF_FFFF, Ordering::Relaxed);
        LAST_ADDR.store(1, Ordering::Relaxed);
        return None;
    }
    let mut log = [0xffu8; EFUSE_SIZE];
    deshuffle(&phys, &mut log);
    let opt = log[RFE_OPTION_OFFSET];
    // 0xFF is the erased value, so an option byte that reads erased was never
    // burned and there is no front-end to select. The tables index off this.
    if opt == 0xFF {
        return None;
    }
    let cut = ((mmio.read32(REG_CHIP_VER) >> 12) & 0xf) as u8;
    let mut mac = [0u8; 6];
    mac.copy_from_slice(&log[MAC_ADDR_OFFSET..MAC_ADDR_OFFSET + 6]);
    Some(EfuseInfo { cut, rfe: opt & 0x1f, pkg: (opt >> 5) & 1, mac })
}
