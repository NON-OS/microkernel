// NONOS Operating System (AGPL-3.0-or-later)

//! The firmware's register mailbox: `rtw89_fw_write_h2c_reg`,
//! `rtw89_fw_read_c2h_reg` and `rtw89_fw_msg_reg` (Linux v6.12 rtw89
//! `fw.c`). Four words go out with the function and length in the first;
//! a counter in UDM1 and a trigger byte tell the firmware; the answer comes
//! back the same way, its trigger cleared once read.

use crate::regs::{field, get, Mmio, Rw};
use crate::wait::until_us;

pub const R_AX_H2CREG_DATA0: u32 = 0x8140;
pub const R_AX_C2HREG_DATA0: u32 = 0x8150;
pub const R_AX_H2CREG_CTRL: u32 = 0x8160;
pub const R_AX_C2HREG_CTRL: u32 = 0x8164;
pub const R_AX_UDM1: u32 = 0x1f4; // counters, byte 1: H2C low nibble, C2H high
const FUNC_MASK: u32 = 0x7f;
const LEN_MASK: u32 = 0xf00;
pub const FUNC_GET_FEATURE: u8 = 3;
pub const FUNC_SCH_TX_EN: u8 = 5;
pub const C2H_PHY_CAP: u8 = 3;
pub const C2H_TX_PAUSE_RPT: u8 = 4;
/// `RTW89_C2H_TIMEOUT`, in microseconds.
const C2H_TIMEOUT_US: u64 = 1_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsgFail {
    /// The firmware never took the last message (5 ms).
    Busy,
    /// No answer in time.
    NoAnswer,
    /// An answer of another function.
    Wrong(u8),
}

/// The mailbox counters, zeroed by each download.
#[derive(Default)]
pub struct Counters {
    pub h2c: u8,
    pub c2h: u8,
}

/// Send function `id` with `words` (header bits in the first word are
/// replaced) and `content_len` bytes, and take the answer of function
/// `want`: its four words.
pub fn msg(
    r: &impl Mmio,
    n: &mut Counters,
    id: u8,
    mut words: [u32; 4],
    content_len: u32,
    want: u8,
) -> Result<[u32; 4], MsgFail> {
    if !until_us(5000, || r.read8(R_AX_H2CREG_CTRL) == 0) {
        return Err(MsgFail::Busy);
    }
    let len = (content_len + 2).div_ceil(4);
    words[0] = (words[0] & !(FUNC_MASK | LEN_MASK)) | field(FUNC_MASK, id as u32);
    words[0] |= field(LEN_MASK, len);
    for (i, w) in words.iter().enumerate() {
        r.write32(R_AX_H2CREG_DATA0 + i as u32 * 4, *w);
    }
    n.h2c = n.h2c.wrapping_add(1);
    r.mask8(R_AX_UDM1 + 1, 0x0f, n.h2c);
    r.write8(R_AX_H2CREG_CTRL, 1);
    if !until_us(C2H_TIMEOUT_US, || r.read8(R_AX_C2HREG_CTRL) != 0) {
        return Err(MsgFail::NoAnswer);
    }
    let back: [u32; 4] = core::array::from_fn(|i| r.read32(R_AX_C2HREG_DATA0 + i as u32 * 4));
    r.write8(R_AX_C2HREG_CTRL, 0);
    n.c2h = n.c2h.wrapping_add(1);
    r.mask8(R_AX_UDM1 + 1, 0xf0, n.c2h);
    let got = get(back[0], FUNC_MASK) as u8;
    (got == want).then_some(back).ok_or(MsgFail::Wrong(got))
}
