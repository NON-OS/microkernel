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

//! The DP rig with pipe A's plane lit beside it, firmware's split of the
//! display buffer given, and the runs a pipe holds read back.

use crate::engine::ddb::Run;
use crate::engine::pipe::Pipe;
use crate::engine::plane_read::ENABLE;
use crate::engine::plane_regs::plane_ctl;
use crate::engine::wm_write::{cur_buf_cfg, plane_buf_cfg};
use crate::model::firmware::T1080;
use crate::tests::dp_rig::{dp_rig, Model};
use crate::tests::edid_blob::edid_1080p_and_xga;
use crate::tests::two_rig::TOTAL;

pub const A: Run = Run { start: 0, end: 400 };
pub const B: (Run, Run) = (Run { start: 400, end: 880 }, Run { start: 880, end: 888 });

pub fn beside(b: (Run, Run)) -> (crate::tests::wire_rig::Rig, Model) {
    let (d, model, _) = dp_rig(T1080, edid_1080p_and_xga((600, 340)), TOTAL);
    let mut m = model.borrow_mut();
    m.set(plane_ctl(Pipe::A, 1), ENABLE);
    m.set(plane_buf_cfg(Pipe::A), A.cfg());
    m.set(plane_buf_cfg(Pipe::B), b.0.cfg());
    m.set(cur_buf_cfg(Pipe::B), b.1.cfg());
    drop(m);
    (d, model)
}

pub fn runs(model: &Model, p: Pipe) -> (Run, Run) {
    let m = model.borrow();
    (Run::of(m.get(plane_buf_cfg(p))), Run::of(m.get(cur_buf_cfg(p))))
}
