// NONOS Operating System (AGPL-3.0-or-later)
//! The power-off tables against rtw88: every entry of rtw8821c.c's
//! act-to-cardemu, cardemu-to-carddis and carddis-to-cardemu tables is written
//! out below with its interface mask, and the driver's tables must be exactly
//! the PCIe ones, in order, with the same command, mask and value.

use crate::pwr::command::{PwrCmd, CMD_END, CMD_POLL, CMD_WRITE};
use crate::pwr::{CARD_DISABLE, CARD_EMULATE, CR_UNPOWERED};

/* rtw88's interface masks: USB BIT(0), SDIO BIT(1), PCI BIT(2), ALL 0x7. */
const U: u8 = 1;
const S: u8 = 2;
const P: u8 = 4;
const A: u8 = 7;
const W: u8 = CMD_WRITE;
const L: u8 = CMD_POLL;

/* (offset, interface mask, command, mask, value), as rtw8821c.c lists them;
 * the SDIO-address entries (RTW_PWR_ADDR_SDIO) carry interface S. */
#[rustfmt::skip]
const ACT_TO_CARDEMU: &[Entry] = &[
    (0x0093, A, W, 0x08, 0), (0x001F, A, W, 0xFF, 0), (0x0049, A, W, 0x02, 0),
    (0x0006, A, W, 0x01, 0x01), (0x0002, A, W, 0x02, 0), (0x10C3, U, W, 0x01, 0),
    (0x0005, A, W, 0x02, 0x02), (0x0005, A, L, 0x02, 0), (0x0020, A, W, 0x08, 0),
    (0x0000, U | S, W, 0x20, 0x20),
];
#[rustfmt::skip]
const CARDEMU_TO_CARDDIS: &[Entry] = &[
    (0x0007, U | S, W, 0xFF, 0x20), (0x0067, A, W, 0x20, 0), (0x0005, P, W, 0x04, 0x04),
    (0x004A, U, W, 0x01, 0), (0x0067, S, W, 0x20, 0), (0x0067, S, W, 0x10, 0),
    (0x004F, S, W, 0x01, 0), (0x0067, S, W, 0x02, 0), (0x0046, S, W, 0x40, 0x40),
    (0x0067, S, W, 0x04, 0), (0x0046, S, W, 0x80, 0x80), (0x0062, S, W, 0x10, 0x10),
    (0x0081, A, W, 0xC0, 0), (0x0005, U | S, W, 0x18, 0x08), (0x0086, S, W, 0x01, 0x01),
    (0x0086, S, L, 0x02, 0), (0x0090, U | P, W, 0x02, 0), (0x0044, S, W, 0xFF, 0),
    (0x0040, S, W, 0xFF, 0x90), (0x0041, S, W, 0xFF, 0), (0x0042, S, W, 0xFF, 0x04),
];
#[rustfmt::skip]
const CARDDIS_TO_CARDEMU: &[Entry] = &[
    (0x0086, S, W, 0x01, 0), (0x0086, S, L, 0x02, 0x02), (0x004A, U, W, 0x01, 0),
    (0x0005, A, W, 0x98, 0), (0x0300, P, W, 0xFF, 0), (0x0301, P, W, 0xFF, 0),
];

/// One rtw88 table entry, and the same entry once its interface is filtered.
type Entry = (u16, u8, u8, u8, u8);
type Pcie = (u16, u8, u8, u8);

fn pcie(tables: &[&[Entry]]) -> Vec<Pcie> {
    let all = tables.iter().flat_map(|t| t.iter());
    all.filter(|e| e.1 & P != 0).map(|e| (e.0, e.2, e.3, e.4)).collect()
}

fn ours(table: &[PwrCmd]) -> Vec<Pcie> {
    assert_eq!(table.last().map(|c| c.cmd), Some(CMD_END), "a table ends with END");
    table[..table.len() - 1].iter().map(|c| (c.offset, c.cmd, c.mask, c.value)).collect()
}

#[test]
fn power_off_is_rtw88s_card_disable_flow_for_pcie() {
    assert_eq!(ours(CARD_DISABLE), pcie(&[ACT_TO_CARDEMU, CARDEMU_TO_CARDDIS]));
}

#[test]
fn power_back_to_card_emulation_is_rtw88s_for_pcie() {
    assert_eq!(ours(CARD_EMULATE), pcie(&[CARDDIS_TO_CARDEMU]));
}

#[test]
fn an_unpowered_mac_reads_0xea_in_reg_cr() {
    assert_eq!(CR_UNPOWERED, 0xEA);
}
