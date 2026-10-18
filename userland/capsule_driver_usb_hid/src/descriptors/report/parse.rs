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

//! A report descriptor (HID 1.11, 6.2.2) walked into the layout of the
//! reports this driver reads, under hard bounds: items, collection depth,
//! pushed globals, report IDs, report size times count, and the bits of any
//! one report. A descriptor past a bound, cut inside an item, or whose
//! collections do not all close is refused whole.

use super::item::next;
use super::keys::Keys;
use super::layout::Layout;
use super::limits::{Refused, MAX_ITEMS};
use super::media::Media;
use super::pointer::Pointer;
use super::walk::Walk;

pub fn parse(desc: &[u8]) -> Result<Layout, Refused> {
    let mut walk = Walk::default();
    let mut at = 0;
    for _ in 0..MAX_ITEMS {
        match next(desc, &mut at)? {
            Some(item) => walk.take(item)?,
            None => return finish(walk),
        }
    }
    if at < desc.len() {
        return Err(Refused);
    }
    finish(walk)
}

/// The layout, keeping each part only when it can be read.
fn finish(walk: Walk) -> Result<Layout, Refused> {
    if walk.depth != 0 || (walk.g.ids && walk.cursors.unnumbered()) {
        return Err(Refused);
    }
    let mut layout = walk.layout;
    layout.ids = walk.g.ids;
    layout.longest = walk.cursors.longest(walk.g.ids);
    layout.keys = layout.keys.filter(Keys::usable);
    layout.media = layout.media.map(|m| m.filter(Media::usable));
    layout.pointer = layout.pointer.filter(Pointer::usable);
    Ok(layout)
}
