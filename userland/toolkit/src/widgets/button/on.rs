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

//! The on look of a plain or quiet toggle: the ordinary raised body, inked
//! in the accent.

use crate::theme::contrast::{at_least, BODY};
use crate::theme::derive::mix;
use crate::theme::{Opaque, Roles};

/// The ink an on toggle labels its `raised` body with: `accent` where that
/// reads at `BODY` on the body with and without the hover wash, else
/// `accent_text`.
pub(super) fn on_ink(r: &Roles) -> Opaque {
    let (fill, wash) = (r.raised.argb(), r.hover_wash.argb());
    let hovered = mix(fill, wash, (wash >> 24) as u8);
    let reads = [fill, hovered].iter().all(|&g| at_least(r.accent.argb(), g, BODY));
    if reads {
        r.accent
    } else {
        r.accent_text
    }
}
