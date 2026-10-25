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

//! A GPU driver's claim taking the boot framebuffer's scanout (L79). Before
//! the claim is made ready the kernel stops drawing, and bootfb's claim, if
//! one stands, is retired, its pages pointed at the sink and released; the
//! stop is latched for the boot in the hold that makes the claim ready, so a
//! claim that fails before then lets the console draw again.

use super::scanout_rule::{at_admit, next, Next};
use super::state::claims;
use super::types::{Claim, ClaimError, Stage};
use crate::hardware::broker::{bootfb, mmio};
use crate::kernel_core::init::framebuffer as console;
use crate::memory::addr::PhysAddr;

/// bootfb's claim, as its holder and stage, if it has one.
fn peek(fb: u64) -> Option<(u32, Stage)> {
    claims().iter().find(|c| c.device_id == fb).map(|c| (c.pid, c.stage))
}

/// The console stopped, and no claim of bootfb left standing or going.
/// Nothing is granted against `by` yet: its claim is still being made.
pub(super) fn take(by: u64) {
    console::stop_drawing();
    let (Some(fb), Some(sink)) = (bootfb::device(), bootfb::sink()) else {
        return;
    };
    loop {
        let seen = peek(fb);
        match (next(seen.map(|s| s.1)), seen) {
            (Next::TakeBack, Some((pid, _))) => take_back(by, pid, fb, sink),
            (Next::Done, _) | (Next::TakeBack, None) => return,
            (Next::Wait, _) => {
                crate::smp::serve_shootdowns();
                core::hint::spin_loop();
            }
        }
    }
}

/// Retired, so the maps under way are recorded first; drained and pointed at
/// the sink; released, the entry going with its own release if one runs. A
/// retire refused means the stage changed since the peek: read again.
fn take_back(by: u64, pid: u32, fb: u64, sink: PhysAddr) {
    let Ok(epoch) = super::leave::retire(pid, fb) else {
        return;
    };
    let pages = mmio::take_back(pid, fb, epoch, sink);
    let _ = super::release::release(pid, fb, epoch);
    let why = "takes the boot framebuffer's scanout: taken from pid";
    crate::log::info!("[BROKER] device {} {} {}, {} pages now a sink", by, why, pid, pages);
}

/// Asked in the hold that makes a claim ready, before anything is written:
/// bootfb's is refused once the scanout was taken, and a covering one asks
/// again while bootfb's stands or is going.
pub(super) fn admits(claims: &[Claim], device_id: u64, covers: bool) -> Result<(), ClaimError> {
    let fb = bootfb::device();
    let stage = claims.iter().find(|c| Some(c.device_id) == fb).map(|c| c.stage);
    at_admit(fb == Some(device_id), console::surrendered(), covers, stage)
}
