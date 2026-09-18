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

//! The four recorded verdicts and the two digests behind them.

use nonos_app_skeleton::PaintBuffer;

use crate::about::data::verify::Recorded;
use crate::about::format_hex::hex_bytes;

use super::super::kv::{pair, ROW_H};
use super::super::metrics::{CARD_PAD, PAIR_H};
use super::verify_boot::{CLAIMS, CLAIM_TEXT, HASH_GAP};
use super::verify_row::row;

pub(super) fn body(fb: &mut PaintBuffer, first: i32, inner: u32, r: &Recorded) {
    let verdicts = [r.kernel_signature, r.secure_boot, r.attestation, r.proof];
    for (i, v) in verdicts.into_iter().enumerate() {
        row(fb, CARD_PAD, first + (i as u32 * ROW_H) as i32, inner, v, CLAIM_TEXT[i], b"");
    }

    // The hashes are the part a reader can take away and check against a build
    // they hold. They are shown in full width rather than in a kv row, because a
    // digest cut to a label column's remainder is worse than useless.
    let hash_y = first + (ROW_H * CLAIMS + HASH_GAP) as i32;
    let mut kernel = [0u8; 64];
    let mut program = [0u8; 64];
    pair(
        fb,
        CARD_PAD,
        hash_y,
        inner,
        b"Kernel BLAKE3",
        hex_bytes(&r.kernel_hash, &mut kernel),
        true,
    );
    let second = hash_y + PAIR_H as i32;
    pair(
        fb,
        CARD_PAD,
        second,
        inner,
        b"Proof program",
        hex_bytes(&r.program_hash, &mut program),
        true,
    );
}
