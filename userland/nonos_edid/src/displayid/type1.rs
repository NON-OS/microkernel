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

//! DisplayID 1.3's detailed timings, type I, data block tag 0x03: twenty
//! byte descriptors laid out as type VII's, but the clock in 10 kHz units
//! (DisplayID 1.3 section 4.4.1; recalled, unverified). Read through type
//! VII's decode with the clock scaled, so the two layouts cannot drift.

use super::section::Section;
use super::type7::decode;
use crate::source::Source;
use crate::timing::Timing;

pub const TAG_TYPE1: u8 = 0x03;

/// Every type I timing in a DisplayID 1.x section.
pub(crate) fn each(s: Section<'_>, f: &mut dyn FnMut(Timing)) {
    if s.v2() {
        return;
    }
    for b in s.blocks().filter(|b| b.tag == TAG_TYPE1) {
        for (t, preferred) in b.body.chunks_exact(20).filter_map(|r| decode(r, 10)) {
            let mut t = Timing::detailed(t, Source::DisplayId);
            t.preferred = preferred;
            f(t);
        }
    }
}
