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

//! One capsule the registry lists: its mark, its name, its pid, the short
//! hex of the measurement its proof was checked against, and whose proof
//! admitted it.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{centre_top, left, mono_left, right, truncate};

use crate::about::data::verify::{Attested, Verdict};
use crate::about::format::u64_decimal;
use crate::about::format_hex::hex_bytes;

use super::super::kv::row_h;
use super::super::metrics::metrics;
use super::super::mono::valid;
use super::append::put;
use super::verify_mark::mark;
use super::verify_row::{mark_w, tint};

const NAME_W: u32 = 210;
const PID_W: u32 = 96;
const NAME_GAP: u32 = 12;
const PUBLISHER: u8 = 255;

pub(super) fn row(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, a: &Attested) {
    let (m, s, rh) = (metrics(), scale(), row_h());
    let (name_w, pid_w, name_gap) = (s.px(NAME_W), s.px(PID_W), s.px(NAME_GAP));
    let r = roles();
    let v = if a.authority == PUBLISHER { Verdict::Unknown } else { Verdict::Holds };
    mark(fb, x, y, v);
    let top = y + centre_top(0, rh, m.body_px);
    let name: &[u8] = if a.name_len == 0 { b"unnamed" } else { &a.name[..a.name_len] };
    let name_x = x + mark_w();
    left(
        fb,
        name_x,
        top,
        &truncate(valid(name), name_w - name_gap, m.body_px),
        r.text_2,
        m.body_px,
    );
    let (mut pid, mut digits) = ([0u8; 24], [0u8; 20]);
    let n = put(&mut pid, b"pid ");
    let n = n + put(&mut pid[n..], u64_decimal(u64::from(a.pid), &mut digits));
    mono_left(fb, name_x + name_w, top, valid(&pid[..n]), r.muted, m.num_px);
    let mut hex = [0u8; 16];
    let short = hex_bytes(&a.measurement, &mut hex);
    mono_left(fb, name_x + name_w + pid_w, top, valid(short), r.text_2, m.num_px);
    right(fb, x + w, top, valid(authority(a.authority)), tint(v), m.body_px);
}

/* The registry's authority byte, in the words a reader needs. */
fn authority(byte: u8) -> &'static [u8] {
    match byte {
        0 => b"vendor",
        PUBLISHER => b"signed only, no proof",
        _ => b"locally built",
    }
}
