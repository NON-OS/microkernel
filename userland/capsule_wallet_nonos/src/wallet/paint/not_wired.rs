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

//! Saying that a screen's backend is not connected.
//!
//! The wallet was built to be honest about this. `Seam::NotWired` exists, all
//! six providers in `pool::active` return it, and six comments across that
//! module say the UI renders the state honestly and that "the swap screen
//! says" so. Nothing in the paint layer ever matched on it. The honesty was
//! designed, documented, produced, and never drawn, so a screen whose backend
//! cannot answer looked exactly like one that could and simply had no data.
//!
//! This is the missing half. It is deliberately plain: a rule, a line of text,
//! and the reason. A screen that cannot act should say so once and get out of
//! the way, not apologise in a coloured box.

use super::scale;
use nonos_app_skeleton::PaintBuffer;

use crate::wallet::theme::{DIM, LINE, MUTED};

/// Height this notice occupies, so a caller can lay out beneath it.
pub const NOTICE_H: u32 = 44;

/// Draw the notice at `y`, spanning `x..x + w`. Returns the y below it.
///
/// `what` names the thing that is not connected, in the reader's terms rather
/// than the code's: "the pool" and "the prover", not "PoolProvider".
pub fn paint(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, what: &str) -> u32 {
    // A hairline above and below rather than a filled panel: this is a
    // statement about the screen, not a control on it.
    fb.fill_rect(x, y, w, 1, LINE());
    fb.fill_rect(x, y + NOTICE_H - 1, w, 1, LINE());

    let _ = fb.text_ttf((x + 2) as i32, (y + 12) as i32, "Not connected", MUTED(), scale::BODY);

    let mut line = alloc::string::String::from(what);
    line.push_str(" is not wired to this build, so this screen cannot act yet.");
    let _ = fb.text_ttf((x + 2) as i32, (y + 27) as i32, &line, DIM(), scale::BODY);

    y + NOTICE_H
}
