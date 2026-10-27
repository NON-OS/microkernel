// NONOS Operating System (AGPL-3.0-or-later)

//! The crystal block's serial interface: `rtw89_mac_write_xtal_si_ax` and
//! `rtw89_mac_read_xtal_si_ax` (Linux v6.12 rtw89 `mac.c`). One command
//! word carries address, data, bit mask and mode with the poll bit set; the
//! block clears the poll bit when done, within 50 ms.

use crate::regs::{field, Mmio};
use crate::wait::until_us;

pub const R_AX_WLAN_XTAL_SI_CTRL: u32 = 0x270;
const ADDR: u32 = 0xff;
const DATA: u32 = 0xff00;
const BITMASK: u32 = 0xff_0000;
const MODE: u32 = 0x300_0000;
pub const CMD_POLL: u32 = 0x8000_0000;
const NORMAL_WRITE: u32 = 0;
const NORMAL_READ: u32 = 1;
/// rtw89's timeout for the poll bit, in microseconds.
const POLL_US: u64 = 50_000;

pub const XTAL_SI_CV: u8 = 0x41;
pub const XTAL_SI_WL_RFC_S0: u8 = 0x80;
pub const XTAL_SI_WL_RFC_S1: u8 = 0x81;
pub const XTAL_SI_ANAPAR_WL: u8 = 0x90;
pub const XTAL_SI_SRAM_CTRL: u8 = 0xa1;
pub const XTAL_SI_XTAL_XMD_2: u8 = 0x24;
pub const XTAL_SI_XTAL_XMD_4: u8 = 0x26;
pub const SRAM2RFC: u8 = 0x80;
pub const GND_SHDN_WL: u8 = 0x40;
pub const SHDN_WL: u8 = 0x20;
pub const RFC2RF: u8 = 0x10;
pub const OFF_EI: u8 = 0x08;
pub const OFF_WEI: u8 = 0x04;
pub const PON_EI: u8 = 0x02;
pub const PON_WEI: u8 = 0x01;
pub const SRAM_DIS: u8 = 0x02;
pub const LDO_LPS: u8 = 0x70;
pub const LPS_CAP: u8 = 0x0f;
pub const RF00: u8 = 0x01;

/// The serial interface did not finish a command in time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XtalTimeout;

/// Write `val` under `mask` at crystal register `offset`.
pub fn write(r: &impl Mmio, offset: u8, val: u8, mask: u8) -> Result<(), XtalTimeout> {
    let cmd = field(ADDR, offset as u32)
        | field(DATA, val as u32)
        | field(BITMASK, mask as u32)
        | field(MODE, NORMAL_WRITE)
        | CMD_POLL;
    r.write32(R_AX_WLAN_XTAL_SI_CTRL, cmd);
    done(r)
}

/// Read crystal register `offset`: the data comes back in byte 1.
pub fn read(r: &impl Mmio, offset: u8) -> Result<u8, XtalTimeout> {
    let cmd = field(ADDR, offset as u32) | field(MODE, NORMAL_READ) | CMD_POLL;
    r.write32(R_AX_WLAN_XTAL_SI_CTRL, cmd);
    done(r)?;
    Ok(r.read8(R_AX_WLAN_XTAL_SI_CTRL + 1))
}

fn done(r: &impl Mmio) -> Result<(), XtalTimeout> {
    let ok = until_us(POLL_US, || r.read32(R_AX_WLAN_XTAL_SI_CTRL) & CMD_POLL == 0);
    ok.then_some(()).ok_or(XtalTimeout)
}
