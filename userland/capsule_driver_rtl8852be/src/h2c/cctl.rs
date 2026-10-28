// NONOS Operating System (AGPL-3.0-or-later)

//! The CMAC control table of the station's MAC ID: `rtw89_fw_h2c_default_
//! cmac_tbl` and `rtw89_fw_h2c_assoc_cmac_tbl` (Linux v6.12 rtw89 `fw.c`,
//! with the 8852B's `H2C_FUNC_MAC_CCTLINFO_UD`). The body is a control
//! word, eight value words and eight mask words (`cctl_tbl`).

pub use super::cctl_tbl::Tbl;
use super::media::PORT;
use super::packet::Cmd;
use super::send::{h2c, SendFail};
use super::Fw;
use crate::dma::DmaMem;
use crate::regs::Mmio;
use crate::ring::tx::TxRing;

pub const CCTL_UD: Cmd = Cmd { cat: 1, class: 5, func: 2 };
/// RTW89_HW_RATE_CCK1 and RTW89_HW_RATE_OFDM6.
pub const CCK1: u32 = 0x0;
pub const OFDM6: u32 = 0x4;

/// `__rtw89_fw_h2c_set_tx_path` with `hal.antenna_tx` unset, as rtw89
/// leaves it unless told an antenna set: TX on path B (`RF_B`), path B
/// mapped to 0.
pub const NTX_PATH: u32 = 2;

/// The default table: TX power mode 0, the TX path, no antenna select,
/// Doppler and power tolerance 0.
pub fn default_tbl() -> Tbl {
    let mut t = Tbl::new();
    t.set(5, 0xe00, 0); // TXPWR_MODE
    t.set(6, 0xf_0000, NTX_PATH).set(6, 0x30_0000, 0).set(6, 0xc0_0000, 0);
    t.set(6, 0x300_0000, 0).set(6, 0xc00_0000, 0); // PATH_MAP_C, _D
    for bit in [28, 29, 30, 31] {
        t.set(6, 1 << bit, 0); // ANTSEL_A..D
    }
    t.set(7, 0xc_0000, 0).set(7, 0xf00_0000, 0); // DOPPLER_CTRL, TXPWR_TOLERENCE
    t
}

/// The table at join and leave for a station never HE: no RTS or data
/// fallback, the lowest RTS retry rate by band, uplink/downlink set, the
/// port, no packet padding, BSR queue size format 0.
pub fn assoc_tbl(five: bool) -> Tbl {
    let mut t = Tbl::new();
    t.set(1, 1 << 25, 1).set(1, 1 << 26, 1); // DISRTSFB, DISDATAFB
    t.set(2, 0xf000_0000, if five { OFDM6 } else { CCK1 });
    t.set(2, 1 << 11, 0).set(3, 1 << 6, 0); // RTS, DATA TXCNT_LMT_SEL
    t.set(7, 1 << 17, 1).set(5, 0x7, PORT as u32); // ULDL, MULTI_PORT_ID
    t.set(7, 0x30_0000, 0).set(7, 0xc0_0000, 0).set(7, 0xc000_0000, 0);
    t.set(8, 0x3000_0000, 0); // padding 20/40/80/160
    t.set(6, 1 << 13, 0); // BSR_QUEUE_SIZE_FORMAT
    t
}

pub fn send<M: DmaMem>(
    r: &impl Mmio,
    q: &mut TxRing<M>,
    fw: &mut Fw,
    t: &Tbl,
) -> Result<(), SendFail> {
    h2c(r, q, fw, CCTL_UD, &t.bytes(), true)
}
