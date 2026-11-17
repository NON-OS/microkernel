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

//! What the bootloader established before the kernel existed, read back.
//!
//! The kernel's own syscall is explicit that it re-verifies none of this: it
//! hands back the record the bootloader wrote into the handoff. So the card says
//! so in its own subtitle. A reader who mistakes this for a live check would draw
//! a stronger conclusion than the evidence supports, and the entire value of this
//! screen is that it does not let them.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{centre_top, left};

use crate::about::data::verify::recorded;

use super::super::card::{self, titled};
use super::super::kv::row_h;
use super::super::metrics::metrics;

pub(super) const CLAIMS: u32 = 4;
const HASH_GAP: u32 = 12;

/// The gap above the digests, at the display scale.
pub(super) fn hash_gap() -> u32 {
    scale().px(HASH_GAP)
}

pub fn height() -> u32 {
    let (m, rh) = (metrics(), row_h());
    card::overhead() + rh + rh * CLAIMS + hash_gap() + m.pair_h * 2
}

// Wording is deliberately past tense throughout: each of these was settled once,
// at a moment that has gone, by a component that is no longer running.
pub(super) const CLAIM_TEXT: [&[u8]; CLAIMS as usize] = [
    b"kernel image signature verified",
    b"UEFI Secure Boot was on",
    b"attestation accepted",
    b"proof verified",
];

pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    let (m, rh) = (metrics(), row_h());
    let muted = roles().muted;
    let inner = card::inner(w);
    let top = titled(fb, 0, y, w, height(), b"Recorded at boot");
    let note = "Measured by the bootloader. Reading it here does not re-check it.";
    left(fb, m.card_pad, top + centre_top(0, rh, m.body_px), note, muted, m.body_px);
    let first = top + rh as i32;
    match recorded() {
        Some(r) => super::verify_boot_body::body(fb, first, inner, &r),
        None => {
            let msg = "the kernel returned no boot record";
            left(fb, m.card_pad, first + centre_top(0, rh, m.body_px), msg, muted, m.body_px);
        }
    }
}
