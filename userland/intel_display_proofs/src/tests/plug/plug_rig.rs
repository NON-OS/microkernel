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

//! A DisplayPort monitor on DDI B, taken over with mode setting, its link
//! trained, and its hot-plug line high and watched; and a mode's shadow.

use super::super::dp_rig::{dp_rig, Model};
use super::super::edid_blob::edid_1080p_and_xga;
use super::super::two_rig::TOTAL;
use super::super::wire_rig::Rig;
use crate::engine::aux_regs::Ddi;
use crate::model::firmware::T1080;
use crate::model::sink::Sink;
use nonos_gfx_serve::{Display, Shadow};

pub fn watched() -> (Rig, Model, Sink) {
    let (mut d, model, sink) = dp_rig(T1080, edid_1080p_and_xga((600, 340)), TOTAL);
    model.borrow_mut().plug_in(Ddi::B, sink.clone());
    d.watch();
    // Firmware lit it untrained (`link_keep_tests`): the first look trains.
    let _ = d.state(0);
    (d, model, sink)
}

/// Two polls: enough for any change of the line to settle.
pub fn settle(d: &mut Rig) {
    d.poll_plugs();
    d.poll_plugs();
}

/// Run `f` with a `w` x `h` shadow, every pixel 0x77.
pub fn with_shadow<T>(w: u32, h: u32, f: impl FnOnce(&Shadow) -> T) -> T {
    let px = vec![0x77; (w * h * 4) as usize];
    f(&Shadow { bytes: &px, width: w, height: h, stride: w * 4, cursor_offset: 0 })
}
