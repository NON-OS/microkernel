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

//! The claims this capsule tested itself, a moment ago, against the live kernel.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{centre_top, left};

use crate::about::data::verify::{live, Live};

use super::super::card::{self, titled};
use super::super::kv::row_h;
use super::super::metrics::metrics;
use super::verify_census::{census, CENSUS_ROWS};
use super::verify_evidence::evidence;
use super::verify_row::row;

const CHECKS: u32 = 4;
const GROUP_GAP: u32 = 14;

pub fn height() -> u32 {
    let (m, rh, group_gap) = (metrics(), row_h(), scale().px(GROUP_GAP));
    card::overhead() + rh * CHECKS + group_gap + rh + rh * CENSUS_ROWS + m.card_foot
}

pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    let (m, rh) = (metrics(), row_h());
    let r = roles();
    let inner = card::inner(w);
    let top = titled(fb, 0, y, w, height(), b"Checked now");
    match live() {
        Some(l) => body(fb, top, inner, &l),
        /*
         * The kernel refusing the process table is itself a fact, and one that
         * must not read as four quiet passes.
         */
        None => {
            let msg = "the kernel did not answer, so nothing below is claimed";
            left(fb, m.card_pad, top + centre_top(0, rh, m.body_px), msg, r.muted, m.body_px);
        }
    }
}

fn body(fb: &mut PaintBuffer, top: i32, inner: u32, l: &Live) {
    let (m, rh, group_gap) = (metrics(), row_h(), scale().px(GROUP_GAP));
    let r = roles();
    for (i, c) in l.checks.iter().enumerate() {
        let mut buf = [0u8; 32];
        row(
            fb,
            m.card_pad,
            top + (i as u32 * rh) as i32,
            inner,
            c.verdict,
            c.claim,
            evidence(c, &mut buf),
        );
    }

    let census_y = top + (rh * CHECKS + group_gap) as i32;
    let head = "Counted, not tested. Nothing here can be wrong.";
    left(fb, m.card_pad, census_y + centre_top(0, rh, m.body_px), head, r.muted, m.body_px);
    census(fb, census_y + rh as i32, inner, &l.census);
}
