// NONOS Operating System (AGPL-3.0-or-later)
//! The WiFi CPU: started with `B_AX_WCPU_EN` while `B_AX_WCPU_FWDL_EN` is
//! set, it raises the H2C path; the firmware header (H2C 1/3/0) raises the
//! download path and names the sections; once their bytes have all come
//! (fw-dl packets on CH12) it reports ready (7) in `R_AX_WCPU_FW_CTRL` and
//! runs, taking H2C commands from then on.

mod download;

pub use download::{header, section_bytes};

use super::state::State;

pub(super) const FW_CTRL: u32 = 0x1e0;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fwdl {
    Off,
    WaitHdr,
    Sections { expected: usize },
    Running,
    Failed(u8),
}

/// One command the running firmware took.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct H2c {
    pub cat: u8,
    pub class: u8,
    pub func: u8,
    pub seq: u8,
    pub rack: bool,
    pub dack: bool,
    pub body: Vec<u8>,
}

impl H2c {
    pub fn is(&self, cat: u8, class: u8, func: u8) -> bool {
        (self.cat, self.class, self.func) == (cat, class, func)
    }
}

pub fn platform(s: &mut State, old: u32, new: u32) {
    let (was, now) = (old & 0x2 != 0, new & 0x2 != 0);
    if now && !was {
        if s.word(FW_CTRL) & 0x1 == 0 {
            s.violations.push("cpu started without the download enable".into());
            return;
        }
        s.fwdl = Fwdl::WaitHdr;
        s.fw_tries += 1;
        s.fw_got.clear();
        s.regs.insert(FW_CTRL, s.word(FW_CTRL) | 0x2); // B_AX_H2C_PATH_RDY
    } else if was && !now {
        s.fwdl = Fwdl::Off;
    }
}
