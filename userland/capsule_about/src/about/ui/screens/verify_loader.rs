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

//! What the kernel found when it checked the bootloader that started it.
//!
//! The card above is the bootloader's word about the kernel; this one is the
//! kernel's word about the bootloader. Both were settled once, at boot, and
//! the subtitle says so for the same reason the card above does.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{centre_top, left};

use crate::about::data::verify::{loader, Loader};
use crate::about::format::u64_decimal;
use crate::about::format_hex::hex_bytes;

use super::super::card::{self, titled};
use super::super::kv::{pair, row_h};
use super::super::metrics::metrics;
use super::append::put;
use super::verify_boot::hash_gap;
use super::verify_row::row;

pub fn height() -> u32 {
    let (m, rh) = (metrics(), row_h());
    card::overhead() + rh * 2 + hash_gap() + m.pair_h * 2
}

pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    let (m, rh) = (metrics(), row_h());
    let muted = roles().muted;
    let inner = card::inner(w);
    let top = titled(fb, 0, y, w, height(), b"Checked by the kernel at boot");
    let note = "The kernel's check of its bootloader. Reading it here does not re-check it.";
    left(fb, m.card_pad, top + centre_top(0, rh, m.body_px), note, muted, m.body_px);
    let l = loader();
    let mut ev = [0u8; 32];
    row(fb, m.card_pad, top + rh as i32, inner, l.verdict, l.claim, evidence(&l, &mut ev));

    /*
     * Shown in full, like the boot record's digests: these are what a reader
     * compares against the release that enrolled the loader.
     */
    let hash_y = top + (rh * 2 + hash_gap()) as i32;
    let (mut mh, mut rh) = ([0u8; 64], [0u8; 64]);
    let (measured, root): (&[u8], &[u8]) = match &l.admitted {
        Some(a) => (hex_bytes(&a.measurement, &mut mh), hex_bytes(&a.root, &mut rh)),
        None => (b"nothing admitted", b"nothing admitted"),
    };
    pair(fb, m.card_pad, hash_y, inner, b"Bootloader Authenticode SHA-256", measured, true);
    pair(fb, m.card_pad, hash_y + m.pair_h as i32, inner, b"Boot root", root, true);
}

/* "epoch 3" for an admitted loader, "code 403" for a refused one. */
fn evidence<'a>(l: &Loader, buf: &'a mut [u8; 32]) -> &'a [u8] {
    let (label, value): (&[u8], u64) = match (l.admitted, l.code) {
        (Some(a), _) => (b"epoch ", a.epoch),
        (None, Some(c)) => (b"code ", u64::from(c)),
        (None, None) => return b"",
    };
    let mut digits = [0u8; 20];
    let mut n = put(&mut buf[..], label);
    n += put(&mut buf[n..], u64_decimal(value, &mut digits));
    &buf[..n]
}
