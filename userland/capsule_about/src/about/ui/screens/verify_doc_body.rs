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

//! The document's contents, once one came back.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{centre_top, left};

use crate::about::data::doc_parse::Doc;
use crate::about::format::u64_decimal;
use crate::about::format_hex::hex_bytes;

use super::super::kv::{kv, pair, row_h};
use super::super::metrics::metrics;
use super::verify_doc_cover::counted;

pub(super) fn body(fb: &mut PaintBuffer, top: i32, inner: u32, doc: &Doc) {
    let (m, rh) = (metrics(), row_h());
    let r = roles();
    let sub = "A TPM signed this root together with the challenge it was given.";
    left(fb, m.card_pad, top + centre_top(0, rh, m.body_px), sub, r.muted, m.body_px);

    /*
     * The root is the whole point of the document, so it gets the card's full
     * width rather than a label column's remainder.
     */
    let root_y = top + rh as i32;
    let mut hex = [0u8; 64];
    pair(
        fb,
        m.card_pad,
        root_y,
        inner,
        b"Capsule registry root",
        hex_bytes(&doc.registry_root, &mut hex),
        true,
    );

    let rows_y = root_y + m.pair_h as i32;
    super::verify_doc_rows::rows(fb, rows_y, inner, doc);
    let mut count = [0u8; 20];
    let mut signed = [0u8; 20];
    let mut sig = [0u8; 20];
    let mut cell = [0u8; 64];
    let n = counted(
        &mut cell,
        u64_decimal(doc.capsule_count as u64, &mut count),
        u64_decimal(doc.attest_len as u64, &mut signed),
        u64_decimal(doc.signature_len as u64, &mut sig),
    );
    kv(fb, m.card_pad, rows_y + (rh * 3) as i32, inner, b"Covered by the key", &cell[..n], true);
}
