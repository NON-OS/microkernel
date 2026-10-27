// NONOS Operating System (AGPL-3.0-or-later)
//! The download itself: the header names the sections, their bytes come,
//! and the firmware reports its status once they all have.

use super::{Fwdl, FW_CTRL};
use crate::model::state::State;

/// The section lengths the header names (`rtw89_fw_hdr_parser_v0`).
fn expected(hdr: &[u8]) -> Option<usize> {
    let w = |i: usize| Some(u32::from_le_bytes(hdr.get(4 * i..4 * i + 4)?.try_into().ok()?));
    let n = ((w(6)? >> 8) & 0xff) as usize;
    (0..n).try_fold(0, |t, i| {
        let s1 = w(9 + 4 * i)?;
        Some(t + (s1 & 0xff_ffff) as usize + if s1 & (1 << 28) != 0 { 8 } else { 0 })
    })
}

pub fn header(s: &mut State, body: &[u8]) {
    match (s.fwdl.clone(), expected(body)) {
        (Fwdl::WaitHdr, Some(n)) => {
            s.fwdl = Fwdl::Sections { expected: n };
            s.fw_hdr = body.to_vec();
            s.regs.insert(FW_CTRL, s.word(FW_CTRL) | 0x4); // B_AX_FWDL_PATH_RDY
        }
        (st, _) => s.violations.push(format!("firmware header in state {st:?}")),
    }
}

pub fn section_bytes(s: &mut State, bytes: &[u8]) {
    let Fwdl::Sections { expected } = s.fwdl else {
        return s.violations.push(format!("download bytes in state {:?}", s.fwdl));
    };
    s.fw_got.extend_from_slice(bytes);
    if s.fw_got.len() >= expected {
        let sts = s.faults.fw_status.unwrap_or(7);
        let ok = sts == 7 && s.fw_got.len() == expected;
        s.fwdl = if ok { Fwdl::Running } else { Fwdl::Failed(sts) };
        s.regs.insert(FW_CTRL, (s.word(FW_CTRL) & !0xe0) | (sts as u32) << 5);
    }
}
