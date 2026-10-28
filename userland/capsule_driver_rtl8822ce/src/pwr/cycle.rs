// NONOS Operating System (AGPL-3.0-or-later)

//! The 8822C's power-off, and the first table of its power-on: rtw88
//! `card_disable_flow_8822c` and `trans_carddis_to_cardemu_8822c`
//! (rtw8822c.c:4594-4887, Linux v6.12), the entries whose interface mask takes
//! PCIe, in order. Register facts from a "GPL-2.0 OR BSD-3-Clause" file, used
//! under BSD-3-Clause (README.md, "Licence"). Every entry is RTW_PWR_CUT_ALL.

use super::command::PwrCmd;

/// `REG_CR` reads this only on a MAC that is powered down (mac.c:291).
pub const CR_UNPOWERED: u8 = 0xEA;

/// `trans_act_to_cardemu_8822c` then `trans_cardemu_to_carddis_8822c`.
pub const CARD_DISABLE: &[PwrCmd] = &[
    PwrCmd::write(0x0093, 0x08, 0x00),
    PwrCmd::write(0x001F, 0xFF, 0x00),
    PwrCmd::write(0x00EF, 0xFF, 0x00),
    PwrCmd::write(0x1045, 0x10, 0x00),
    PwrCmd::write(0x0049, 0x02, 0x00),
    PwrCmd::write(0x0006, 0x01, 0x01),
    PwrCmd::write(0x0002, 0x02, 0x00),
    PwrCmd::write(0x0005, 0x02, 0x02),
    PwrCmd::poll(0x0005, 0x02, 0x00), // the MAC has left the active state
    // card emulation to card disable
    PwrCmd::write(0x0067, 0x20, 0x00),
    PwrCmd::write(0x0081, 0xC0, 0x00),
    PwrCmd::write(0x0090, 0x02, 0x00),
    PwrCmd::write(0x0092, 0xFF, 0x20),
    PwrCmd::write(0x0093, 0xFF, 0x04),
    PwrCmd::write(0x0005, 0x04, 0x04),
    PwrCmd::end(),
];

/// `trans_carddis_to_cardemu_8822c`.
pub const CARD_EMULATE: &[PwrCmd] = &[
    PwrCmd::write(0x002E, 0x04, 0x04),
    PwrCmd::write(0x002D, 0x01, 0x00),
    PwrCmd::write(0x007F, 0x80, 0x00),
    PwrCmd::write(0x0005, 0x98, 0x00),
    PwrCmd::end(),
];
