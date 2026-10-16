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

//! A card's second line: a download's progress, a tier's model, or the listing's origin.

use nonos_app_skeleton::PaintBuffer;

use crate::status_wire::Status;
use crate::store::listing::Listing;
use crate::store::model_card::model_line;
use crate::store::model_weights::MODEL_WEIGHTS;
use crate::store::route_offer::progress_line;
use crate::store::theme::{ACCENT, MUTED};

use super::super::metrics::SMALL_PX;
use super::super::text;

pub(super) fn subtitle(
    fb: &mut PaintBuffer,
    l: &Listing,
    fetch: Option<Status>,
    text_x: u32,
    y: u32,
) {
    /*
     * A Qwen tier says what it downloads and the memory it needs, and while
     * its model comes, how far it has come (`route_offer::progress_line`).
     */
    match (fetch, model_line(&l.id, MODEL_WEIGHTS)) {
        (Some(s), _) => {
            text::line(fb, text_x, y as i32 + 34, progress_line(&s).as_bytes(), ACCENT, SMALL_PX)
        }
        (None, Some(line)) => text::line(fb, text_x, y as i32 + 34, &line, MUTED, SMALL_PX),
        (None, None) => text::line(fb, text_x, y as i32 + 34, l.source.origin(), MUTED, SMALL_PX),
    };
}
