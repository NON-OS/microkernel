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

//! A mode set on a pipe lit beside another keeps the buffer runs it has,
//! so its plane never reads blocks the other pipe's plane is using; alone,
//! it takes the whole buffer. A lit pipe that firmware gave no buffer is
//! refused rather than given blocks another pipe may hold.

use nonos_gfx_serve::Display;
use nonos_gfx_wire::errno::EINVAL;

use super::share_rig::{beside, runs, A, B};
use crate::engine::ddb::Run;
use crate::engine::pipe::Pipe;
use crate::engine::plane_regs::plane_ctl;
use crate::engine::wm_write::plane_wm;
use crate::tests::plug::plug_rig::with_shadow;

#[test]
fn beside_another_lit_pipe_the_mode_set_keeps_its_own_runs() {
    let (mut d, model) = beside(B);
    with_shadow(1024, 768, |s| assert_eq!(d.set_mode(0, 1024, 768, s), Ok(())));
    assert_eq!(runs(&model, Pipe::B), B, "its own runs, not the whole buffer");
    assert_eq!(runs(&model, Pipe::A).0, A, "the other pipe's untouched");
    let wm0 = model.borrow().get(plane_wm(Pipe::B, 0));
    assert!(wm0 >> 31 != 0 && wm0 & 0x3FF < B.0.len(), "level 0 on, inside its own share");
}

#[test]
fn alone_the_mode_set_takes_the_whole_buffer() {
    let (mut d, model) = beside(B);
    model.borrow_mut().set(plane_ctl(Pipe::A, 1), 0);
    with_shadow(1024, 768, |s| assert_eq!(d.set_mode(0, 1024, 768, s), Ok(())));
    assert_eq!(runs(&model, Pipe::B).0, Run { start: 0, end: 864 });
}

#[test]
fn a_lit_pipe_with_no_buffer_of_its_own_is_refused() {
    let (mut d, model) = beside((Run::default(), Run::default()));
    with_shadow(1024, 768, |s| assert_eq!(d.set_mode(0, 1024, 768, s), Err(EINVAL)));
    assert_eq!(runs(&model, Pipe::A).0, A, "nothing taken from the other pipe");
}
