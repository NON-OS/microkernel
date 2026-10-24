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

//! A program stopped by Ctrl+C, ended here the way one that exits on its
//! own ends: it never gets to undo the screen modes it set (the alternate
//! screen, a hidden cursor, a colour left on). What it wrote after the key
//! is dropped, as a tty drops it on an interrupt, but for a boxed run's
//! report, which is drawn last, or kept while another program holds the
//! screen, as at any end.

use super::box_hold::BoxHold;
use super::capture::Capture;
use crate::command::output::Output;
use crate::jobs::JobProgress;

pub(super) fn stop_external(
    pid: u32,
    capture: Option<&Capture>,
    mut boxed: Option<&mut BoxHold>,
    out: &mut Output<'_>,
    leave_modes: bool,
) -> JobProgress {
    /* Stopped already, its report kept for the shell's screen. */
    if let Some(progress) = super::box_wait::waiting(boxed.as_deref_mut(), out, leave_modes) {
        return progress;
    }
    super::external_io::discard_output(pid, boxed.as_deref_mut());
    if !leave_modes {
        out.program_ended();
    }
    if let Some(capture) = capture {
        capture.interrupted(out);
    }
    out.writeln(b"interrupted");
    super::box_wait::stopped(boxed, out, leave_modes)
}
