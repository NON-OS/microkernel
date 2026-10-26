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

//! Serial formatting for the pre-iretq audit. One success line and a
//! short failure line. The orchestrator owns the policy; this
//! module owns the bytes.

use super::gs_state::GsState;
use crate::sys::serial::Line;

pub(super) fn fail(reason: &[u8], v: u64) {
    let mut l = Line::new();
    l.str(b"[USER-PROOF] FAIL ").str(reason).str(b" v=").hex_lower(v).end();
}

pub(super) fn ok(cr3: u64, rip: u64, rsp: u64, rsp0: u64, gs: GsState, df: u64, pf: u64, gp: u64) {
    let mut l = Line::new();
    l.str(b"[USER-PROOF] OK cr3=").hex_lower(cr3).str(b" rip=").hex_lower(rip);
    l.str(b" rsp=").hex_lower(rsp).str(b" rsp0=").hex_lower(rsp0);
    l.str(b" gsrsp0=").hex_lower(gs.rsp0).str(b" gs=").hex_lower(gs.base);
    l.str(b" kgs=").hex_lower(gs.kernel_base).str(b" istDF=").hex_lower(df);
    l.str(b" istPF=").hex_lower(pf).str(b" istGP=").hex_lower(gp).end();
}
