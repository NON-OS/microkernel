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

//! The tonal kind's colours: the window's ground tinted towards the accent.

use crate::theme::contrast::{at_least, BODY};
use crate::theme::derive::mix;
use crate::theme::{Opaque, Roles};

/// How far the tonal body moves from `bg` towards `accent`, in 255ths, before
/// any thinning.
const TINT: u8 = 0x30;

/// How much the tint thins per step while the label does not read.
const STEP: u8 = 4;

/// The tonal body and its label ink: `bg` moved `TINT` towards `accent`, or as
/// much less as it takes for a label to read at `BODY` on it with and without
/// the hover wash. The label is `accent_text` when that reads, else `text`.
pub(super) fn tonal(r: &Roles) -> (Opaque, Opaque) {
    let wash = r.hover_wash.argb();
    let reads = |ink: Opaque, fill: u32| {
        let hovered = mix(fill, wash, (wash >> 24) as u8);
        [fill, hovered].iter().all(|&g| at_least(ink.argb(), g, BODY))
    };
    let mut tint = TINT;
    loop {
        let fill = mix(r.bg.argb(), r.accent.argb(), tint);
        let ink = [r.accent_text, r.text].into_iter().find(|&i| reads(i, fill));
        match ink {
            Some(ink) => return (Opaque::new(fill), ink),
            None if tint < STEP => return (Opaque::new(fill), r.text),
            None => tint -= STEP,
        }
    }
}
