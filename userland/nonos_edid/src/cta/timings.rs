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

//! Every timing a CTA-861 extension names: its detailed timings, then
//! the formats of its video data blocks.

use super::blocks::Cta;
use super::vic::vic;
use super::video::{svd, TAG_VIDEO};
use crate::source::Source;
use crate::timing::Timing;

pub(crate) fn each(c: Cta<'_>, f: &mut dyn FnMut(Timing)) {
    for d in c.detailed() {
        f(Timing::detailed(d, Source::CtaDetailed));
    }
    for b in c.data_blocks().filter(|b| b.tag == TAG_VIDEO && b.extended.is_none()) {
        for (code, native) in b.body.iter().filter_map(|&s| svd(s)) {
            let Some(v) = vic(code) else { continue };
            let mut t =
                Timing::nominal(v.width as u32, v.height as u32, v.hz as u32, Source::CtaVic);
            t.interlaced = v.interlaced;
            t.native = native;
            f(t);
        }
    }
}
