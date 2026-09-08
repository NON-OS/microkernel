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

//! The two claims the document settles on its own.
//!
//! The challenge echo is the anti-replay check and the only part of the document
//! this capsule can verify without the key; the completeness flag is the machine
//! saying whether it still knows everything it is running.

use nonos_app_skeleton::PaintBuffer;

use crate::about::data::doc_parse::Doc;
use crate::about::data::verify::Verdict;

use super::super::kv::ROW_H;
use super::super::metrics::CARD_PAD;
use super::verify_row::row;

pub(super) fn rows(fb: &mut PaintBuffer, rows_y: i32, inner: u32, doc: &Doc) {
    row(
        fb,
        CARD_PAD,
        rows_y,
        inner,
        Verdict::from_bool(doc.challenge_echoed),
        b"challenge came back unchanged",
        b"",
    );
    row(
        fb,
        CARD_PAD,
        rows_y + ROW_H as i32,
        inner,
        Verdict::from_bool(doc.registry_complete),
        b"machine recorded everything it runs",
        b"",
    );
}
