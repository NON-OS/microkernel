// NONOS Operating System (AGPL-3.0-or-later)
//! The register mailbox as the running firmware serves it (`rtw89_fw_msg_reg`):
//! it takes the four H2C words, clears `R_AX_H2CREG_CTRL`, and answers in
//! the C2H words with `R_AX_C2HREG_CTRL` raised. GET_FEATURE (3) answers
//! PHY_CAP (3); SCH_TX_EN (5) applies the enable under the mask to
//! `R_AX_CTN_TXEN` and answers TX_PAUSE_RPT (4). A firmware that is not
//! running takes nothing.

use super::cpu::Fwdl;
use super::state::State;

const H2C_DATA0: u32 = 0x8140;
const C2H_DATA0: u32 = 0x8150;
const H2C_CTRL: u32 = 0x8160;
const C2H_CTRL: u32 = 0x8164;
const CTN_TXEN: u32 = 0xc348;

pub fn take(s: &mut State) {
    if s.fwdl != Fwdl::Running || s.faults.mute {
        return;
    }
    let w: [u32; 4] = core::array::from_fn(|i| s.word(H2C_DATA0 + 4 * i as u32));
    s.mailbox.push(w);
    s.regs.insert(H2C_CTRL, s.word(H2C_CTRL) & !0xff);
    if s.word(C2H_CTRL) & 0xff != 0 {
        s.violations.push("mailbox message before the last answer was taken".into());
    }
    let answer = match w[0] & 0x7f {
        3 => {
            let mut a = s.phy_cap;
            a[0] = (a[0] & !0x7f) | 3;
            a
        }
        5 => {
            let (en, mask) = (w[0] >> 16, w[1] & 0xffff);
            let old = s.word(CTN_TXEN);
            s.regs.insert(CTN_TXEN, (old & !mask) | (en & mask));
            [4, 0, 0, 0]
        }
        f => {
            s.violations.push(format!("mailbox function {f}"));
            return;
        }
    };
    for (i, v) in answer.iter().enumerate() {
        s.regs.insert(C2H_DATA0 + 4 * i as u32, *v);
    }
    s.regs.insert(C2H_CTRL, (s.word(C2H_CTRL) & !0xff) | 1);
}
