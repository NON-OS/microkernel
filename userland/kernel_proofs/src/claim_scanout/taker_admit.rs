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

//! A covering claim's admit in the takeover model (L79), pending::admit's
//! one hold of the table's lock: the Making check, the scanout check, the
//! wipe record, Ready and GONE latched; refused, nothing written. Where the
//! start has G1's holder exit, that exit lands before this admit.

use super::events::{ADMITTED, ASKED_AGAIN, ORPHANED};
use super::knob::Knob;
use super::scanout_rule::at_admit;
use super::taker::{ENDS, STOP, WITHDRAW};
use super::types::{ClaimError, Stage};
use super::world::{code, stage, END};
use super::world_check::{SPURIOUS, WIPE_LEFT};
use super::world_state::World;

pub(super) fn admit(w: &mut World, g: usize, k: Knob) -> bool {
    if g == 0 && w.cfg.doomed && w.pc[6] < END {
        return false;
    }
    let making = w.gpu[g] == Some(code(Stage::Making));
    let answer = if making { scanout(w, g, k) } else { Err(ClaimError::NotClaimed) };
    w.g[g].ok = answer.is_ok();
    match answer {
        Ok(()) => {
            w.g[g].wiped = true;
            w.gpu[g] = Some(code(Stage::Ready));
            w.ever_in = true;
            w.gone |= k != Knob::Control;
            w.events |= ADMITTED;
        }
        Err(e) => {
            if w.g[g].wiped {
                w.fail(WIPE_LEFT);
            }
            if e == ClaimError::Resetting && !w.g[g].late {
                w.fail(SPURIOUS);
            }
            w.events |= if e == ClaimError::Resetting { ASKED_AGAIN } else { ORPHANED };
        }
    }
    w.pc[g] = match k {
        Knob::Control if w.g[g].ok => END,
        Knob::Control => WITHDRAW,
        Knob::TakeAfterAdmit => STOP,
        _ => ENDS,
    };
    true
}

/// scanout::admits with bootfb's stage as the table holds it, after the
/// wipe record (ScanoutAfterWipe), or not at all (Control).
fn scanout(w: &mut World, g: usize, k: Knob) -> Result<(), ClaimError> {
    w.g[g].wiped |= k == Knob::ScanoutAfterWipe;
    if k == Knob::Control {
        return Ok(());
    }
    let fb = w.fb.map(|e| stage(e.stage));
    let fb = fb.filter(|&s| !(k == Knob::LeavingDone && s == Stage::Leaving));
    at_admit(false, w.gone, true, fb)
}
