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
//! What every wallet screen does around its own content: the presses the
//! frame placed (back, dismiss, the pinned actions), and the lead sentence.

use nonos_app_skeleton::PaintBuffer;

use super::hits::{self, Press};
use crate::wallet::etna::frame_spec::FrameLayout;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;

/// Back, dismiss, and each enabled footer action, as presses.
pub fn edges(l: &FrameLayout, enabled: &[bool]) {
    if let Some(back) = l.back {
        hits::put(Press::Back, back);
    }
    if let Some(d) = l.dismiss {
        hits::put(Press::Dismiss, d);
    }
    for (i, on) in enabled.iter().enumerate() {
        if *on {
            hits::put(Press::Footer(i as u8), l.footer[i]);
        }
    }
}

/// The screen's one lead sentence; returns where the content goes on.
pub fn lead(fb: &mut PaintBuffer, c: Rect, text: &str) -> u32 {
    c.y + wrapped(fb, c.x as i32, c.y as i32, c.w as i32, Role::Lead, text, TEXT_3) as u32 + GAP
}
