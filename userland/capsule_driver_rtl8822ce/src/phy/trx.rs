// NONOS Operating System (AGPL-3.0-or-later)

//! Which RF paths transmit and receive: rtw88 `rtw8822c_config_trx_mode`
//! (rtw8822c.c:2435-2546) with its 1-stream transmit and CCK transmit on
//! path A, as `rtw8822c_phy_set_param` calls it. Register facts from
//! rtw8822c.h.

use super::bits::write32_mask;
use super::igi::toggle_igi;
use super::power::bb_reset;
use crate::regs::Mmio;

/// `enum rtw_bb_path`.
pub const BB_PATH_A: u8 = 1 << 0;
pub const BB_PATH_B: u8 = 1 << 1;
pub const BB_PATH_AB: u8 = BB_PATH_A | BB_PATH_B;

/// Configure `tx` and `rx` (each a `BB_PATH_*`), then retoggle the IGI.
pub fn config<M: Mmio>(mmio: &M, tx: u8, rx: u8) {
    let code = |on: bool| if on { 0x33312 } else { 0x11111 };
    write32_mask(mmio, 0x1800, 0xF_FFFF, code((tx | rx) & BB_PATH_A != 0)); // REG_ORITXCODE
    write32_mask(mmio, 0x4100, 0xF_FFFF, code((tx | rx) & BB_PATH_B != 0)); // REG_ORITXCODE2
    rx_path(mmio, rx);
    // CCK on path A (`rtw8822c_config_cck_tx_path`).
    write32_mask(mmio, 0x1A04, 0xF000_0000, 0x8); // REG_RXCCKSEL
    bb_reset(mmio);
    ofdm_tx_path(mmio, tx);
    bb_reset(mmio);
    bb_reset(mmio);
    toggle_igi(mmio);
}

fn rx_path<M: Mmio>(mmio: &M, rx: u8) {
    let both = u32::from(rx == BB_PATH_AB);
    if rx == BB_PATH_AB {
        write32_mask(mmio, 0x1A2C, 0x0060_0000, 1); // REG_CCANRX
        write32_mask(mmio, 0x1A2C, 0x0006_0000, 1);
    } else if rx == BB_PATH_A || rx == BB_PATH_B {
        write32_mask(mmio, 0x1A2C, 0x0006_0000, 0);
        write32_mask(mmio, 0x1A2C, 0x0060_0000, 0);
    }
    match rx {
        BB_PATH_A => write32_mask(mmio, 0x1A04, 0x0F00_0000, 0x0),
        BB_PATH_B => write32_mask(mmio, 0x1A04, 0x0F00_0000, 0x5),
        BB_PATH_AB => write32_mask(mmio, 0x1A04, 0x0F00_0000, 0x1),
        _ => {}
    }
    if rx == BB_PATH_A || rx == BB_PATH_B || rx == BB_PATH_AB {
        write32_mask(mmio, 0x1D30, 0x300, both); // REG_RXFNCTL
        write32_mask(mmio, 0x1D30, 0x60_0000, both);
        write32_mask(mmio, 0x0C44, 1 << 17, both); // REG_AGCSWSH
        write32_mask(mmio, 0x0C54, 1 << 20, both); // REG_ANTWTPD
        write32_mask(mmio, 0x0C38, 1 << 24, both); // REG_MRCM
    }
    write32_mask(mmio, 0x0824, 0x0F00_0000, u32::from(rx));
    write32_mask(mmio, 0x0824, 0x000F_0000, u32::from(rx));
}

/// The OFDM transmit path, its 1-stream selection on path A.
fn ofdm_tx_path<M: Mmio>(mmio: &M, tx: u8) {
    let (antmap, lg_mask, lg) = match tx {
        BB_PATH_A => (0x11, 0xFF, 0x0),
        BB_PATH_B => (0x12, 0xFF, 0x0),
        _ => (0x31, 0xFFFF, 0x0400),
    };
    write32_mask(mmio, 0x0820, 0xFF, antmap); // REG_ANTMAP0
    write32_mask(mmio, 0x1E2C, lg_mask, lg); // REG_TXLGMAP
}
