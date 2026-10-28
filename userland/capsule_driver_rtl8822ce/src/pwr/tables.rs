// NONOS Operating System (AGPL-3.0-or-later)

//! The 8822C's card-emulation-to-active transition: rtw88
//! `trans_cardemu_to_act_8822c` (rtw8822c.c:4637, Linux v6.12), the entries
//! whose interface mask takes PCIe, in order. Register facts from a "GPL-2.0
//! OR BSD-3-Clause" file, used under BSD-3-Clause (README.md, "Licence").

use super::command::PwrCmd;

/// Bring the MAC from card emulation to active.
pub const CARD_ENABLE: &[PwrCmd] = &[
    PwrCmd::write(0x0005, 0x1C, 0x00),
    PwrCmd::write(0x0075, 0x01, 0x01),
    PwrCmd::poll(0x0006, 0x02, 0x02), // the power-ready bit
    PwrCmd::write(0x0075, 0x01, 0x00),
    PwrCmd::write(0x002E, 0x08, 0x00),
    PwrCmd::write(0x0006, 0x01, 0x01),
    PwrCmd::write(0x0005, 0x18, 0x00),
    PwrCmd::write(0x1018, 0x04, 0x04),
    PwrCmd::write(0x0005, 0x01, 0x01),
    PwrCmd::poll(0x0005, 0x01, 0x00), // the enable has settled
    PwrCmd::write(0x0074, 0x20, 0x20),
    PwrCmd::write(0x0071, 0x10, 0x00),
    PwrCmd::write(0x0062, 0xE0, 0xE0),
    PwrCmd::write(0x0061, 0xE0, 0x00),
    PwrCmd::write(0x001F, 0xC0, 0x80),
    PwrCmd::write(0x00EF, 0xC0, 0x80),
    PwrCmd::write(0x1045, 0x10, 0x10),
    PwrCmd::write(0x0010, 0x04, 0x04),
    PwrCmd::write(0x1064, 0x02, 0x02),
    PwrCmd::end(),
];
