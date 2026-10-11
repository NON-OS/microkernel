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

//! Every timing the base block names, in the order E-EDID 1.4 lays them
//! out: established, standard, then the four descriptors.

use super::descriptor::{Descriptor, TAG_CVT, TAG_EST3, TAG_STANDARD};
use super::{cvt3, est3, established, standard};
use crate::edid::Edid;
use crate::source::Source;
use crate::timing::Timing;

impl Edid<'_> {
    pub(crate) fn base_timings(&self, f: &mut dyn FnMut(Timing)) {
        let b = self.base();
        let new = self.at_least_1_3();
        established::each(b, f);
        standard::each(&b[38..54], new, f);
        let mut first = true;
        for d in self.descriptors() {
            match d {
                Descriptor::Timing(t) => {
                    let mut t = Timing::detailed(t, Source::Detailed);
                    // Section 3.10.1: the first detailed timing is the
                    // preferred one in every 1.3 and 1.4 structure.
                    t.preferred = first;
                    first = false;
                    f(t);
                }
                Descriptor::Display { tag: TAG_STANDARD, bytes } => {
                    standard::each(&bytes[5..17], new, f)
                }
                Descriptor::Display { tag: TAG_CVT, bytes } => cvt3::each(bytes, f),
                Descriptor::Display { tag: TAG_EST3, bytes } => est3::each(bytes, f),
                Descriptor::Display { .. } => {}
            }
        }
    }
}
