// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The two-output rig for the control proofs: the eDP panel's PWM set up,
//! and, when asked, pipe B's monitor on HDMI with DDC/CI on DDI B's pins.

use crate::display::{brightness_answer, Rights};
use crate::engine::backlight::BLC_PWM_PCH_CTL2;
use crate::engine::pipe::Transcoder;
use crate::engine::regs::trans_ddi_func_ctl;
use crate::model::mccs::Mccs;
use crate::tests::two_rig::two_rig;
use crate::tests::wire_rig::Rig;

pub const HDMI: [u8; 4] = 0x8000_0001u32.to_le_bytes();

pub fn rig(ddc: bool) -> (Rig, std::rc::Rc<std::cell::RefCell<crate::model::Gen9>>) {
    let (d, model, _) = two_rig();
    let mut m = model.borrow_mut();
    m.set(BLC_PWM_PCH_CTL2, 4800 << 16);
    if ddc {
        let ctl = trans_ddi_func_ctl(Transcoder::B);
        let v = m.get(ctl) & !(7 << 24);
        m.set(ctl, v);
        m.ddc.mccs = Some((5, Mccs::with(&[(0x10, 70, 100), (0x12, 50, 80), (0x60, 0x0F, 0x12)])));
    }
    drop(m);
    (d, model)
}

pub fn ask(d: &mut Rig, req: &[u8], may: bool) -> (i32, Vec<u8>) {
    let mut out = [0u8; 512];
    let n = brightness_answer(d, 2, req, Rights { set: may, read_controls: true }, &mut out);
    (i32::from_le_bytes(out[..4].try_into().unwrap()), out[4..n].to_vec())
}

/// As a sender with no right to read a monitor's controls.
pub fn ask_unread(d: &mut Rig, req: &[u8]) -> i32 {
    let mut out = [0u8; 512];
    let may = Rights { set: false, read_controls: false };
    brightness_answer(d, 2, req, may, &mut out);
    i32::from_le_bytes(out[..4].try_into().unwrap())
}
