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

//! The proofs panel: every STARK proof and boot measurement this loader
//! handles, shown once the kernel is verified and the evidence for the
//! kernel's check of the loader is gathered, just before the handoff.

use uefi::table::boot::BootServices;

use super::boot_log::log_row;
use super::draw::draw_panel;
use super::input::Proofs;
use super::rows::rows;
use crate::display::boot::draw_log_card;
use crate::display::fx::clear_region;
use crate::display::gop::is_initialized;
use crate::display::log_panel::log_security;

/// How long the panel stays up before the boot moves on: 1.5 s.
const HOLD_US: usize = 1_500_000;

/// Write every line to the boot log, then, with a screen, draw the panel and
/// hold it briefly. Nothing here waits on input.
pub fn show_proofs(bs: &BootServices, gop: bool, p: &Proofs<'_>) {
    let rows = rows(p);
    log_security(b"PROOFS");
    for row in &rows {
        log_row(row);
    }
    if !gop || !is_initialized() {
        return;
    }
    let f = draw_panel(&rows);
    bs.stall(HOLD_US);
    /* The handoff logs a few lines more; give the log card its area back. */
    clear_region(f.x, f.y, f.w, f.h);
    draw_log_card();
}
