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

//! What firmware left lit: for each pipe, the transcoder feeding it, the
//! timing it runs, the pipe's source size and its primary plane. Nothing
//! is written; a pipe whose transcoder is off or whose plane is dark is
//! not an output.

use super::access::{field, Mmio};
use super::pipe::{Pipe, Transcoder};
use super::plane_read::PlaneState;
use super::{regs, timing};
use nonos_edid::Detailed;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Output {
    pub pipe: Pipe,
    pub transcoder: Transcoder,
    pub timing: Detailed,
    /// PIPESRC: the picture the planes compose, which the takeover keeps.
    pub width: u32,
    pub height: u32,
    pub plane: PlaneState,
}

/// TRANS_CONF bit 30, the transcoder's state, not its enable request.
fn running(m: &mut dyn Mmio, t: Transcoder) -> bool {
    m.read(regs::trans_conf(t)) & (1 << 30) != 0
}

/// The transcoder feeding `p`: the eDP transcoder when it is running and
/// selects `p`, else the pipe's own.
fn feeding(m: &mut dyn Mmio, p: Pipe) -> Transcoder {
    let edp = m.read(regs::trans_ddi_func_ctl(Transcoder::Edp));
    let edp_on = edp & (1 << 31) != 0 && running(m, Transcoder::Edp);
    match Transcoder::edp_pipe(edp) {
        Some(q) if edp_on && q == p => Transcoder::Edp,
        _ => Transcoder::of(p),
    }
}

pub fn read_output(m: &mut dyn Mmio, p: Pipe) -> Option<Output> {
    let transcoder = feeding(m, p);
    if !running(m, transcoder) {
        return None;
    }
    let timing = timing::read(m, transcoder)?;
    let src = m.read(regs::pipe_src(p));
    let (width, height) = (field(src, 28, 16) + 1, field(src, 11, 0) + 1);
    let plane = PlaneState::read(m, p);
    plane.enabled().then_some(Output { pipe: p, transcoder, timing, width, height, plane })
}

/// Every lit output, pipe A first; output n of the wire is the nth.
pub fn read_outputs(m: &mut dyn Mmio) -> [Option<Output>; 3] {
    Pipe::ALL.map(|p| read_output(m, p))
}
