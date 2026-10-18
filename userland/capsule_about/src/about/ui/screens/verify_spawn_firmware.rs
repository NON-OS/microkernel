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

//! The registry's firmware entry, drawn in the capsule rows' columns but as
//! what it is: no mark, since a digest is a measurement and not a check that
//! passed, no pid, since no process holds the one it is filed under, and the
//! moment it was taken in place of whose proof admitted it.

use nonos_app_skeleton::PaintBuffer;

use crate::about::data::proofs::firmware::{FIRMWARE_LABEL, FIRMWARE_WHEN};
use crate::about::data::verify::Attested;
use crate::about::format_hex::hex_bytes;
use crate::about::theme::{FOREGROUND, MUTED};

use super::super::kv::ROW_H;
use super::super::metrics::{BODY_PX, NUM_PX};
use super::super::text::{fit, line, mono, right, top_of};
use super::verify_row::MARK_W;
use super::verify_spawn_row::{NAME_W, PID_W};

pub(super) fn row(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, a: &Attested) {
    let top = top_of(y, ROW_H, BODY_PX);
    let name_x = x + MARK_W;
    line(fb, name_x, top, FIRMWARE_LABEL, FOREGROUND, BODY_PX);
    let none = fit(fb, b"no process", BODY_PX, PID_W - 8);
    line(fb, name_x + NAME_W, top, none, MUTED, BODY_PX);
    let mut hex = [0u8; 16];
    let short = hex_bytes(&a.measurement, &mut hex);
    mono(fb, name_x + NAME_W + PID_W, top, short, FOREGROUND, NUM_PX);
    right(fb, x + w, top, FIRMWARE_WHEN, MUTED, BODY_PX);
}
