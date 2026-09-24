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

//! Why the selected listing can or cannot be installed.

use nonos_app_skeleton::PaintBuffer;

use crate::store::market::{Readiness, GATES};
use crate::store::theme::{ACCENT, DANGER, MUTED, OK};
use crate::store::verdict::Verdict;

use super::metrics::{BODY_PX, GATE_ROW_H, SMALL_PX};
use super::text;

/// The column the verdicts line up in, left of the pane's right edge by
/// enough that the longest label above still clears it.
const MARK_X: u32 = 190;

pub fn paint(fb: &mut PaintBuffer, x: u32, mut top: i32, r: &Readiness) {
    let verdict = Verdict::of(r);
    let hue = match verdict {
        Verdict::Ready => OK,
        Verdict::Installed => ACCENT,
        Verdict::Blocked => DANGER,
    };
    text::line(fb, x, top, verdict.label(), hue, BODY_PX);
    top += 24;
    if verdict == Verdict::Installed {
        // The package gate below will read as a failure.
        text::line(fb, x, top, b"in this image; nothing to fetch", MUTED, SMALL_PX);
    }
    top += 24;
    for (label, pass) in GATES.iter().zip(r.gates.iter()) {
        let mark: &[u8] = if *pass { b"pass" } else { b"fail" };
        let hue = if *pass { OK } else { DANGER };
        text::line(fb, x, top, label, MUTED, SMALL_PX);
        text::line(fb, x + MARK_X, top, mark, hue, SMALL_PX);
        top += GATE_ROW_H as i32;
    }
}
