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

//! Step 1, the layout: every flag is 0 or 1, the vendor is known, the PCR
//! list is exactly K1's, every length is within its cap, and
//! nothing is left over or cut short.

use nsh_attest::{FormatFault, Refusal};

use super::refused;
use crate::build::Spec;

const VALUE: Refusal = Refusal::Format(FormatFault::Value);

#[test]
fn flags_other_than_0_or_1_are_refused() {
    assert_eq!(refused(Spec { doc: |p| p.flags[0] = 2, ..Spec::default() }), VALUE);
    assert_eq!(refused(Spec { doc: |p| p.flags[2] = 2, ..Spec::default() }), VALUE);
}

#[test]
fn an_unknown_iommu_vendor_is_refused() {
    assert_eq!(refused(Spec { doc: |p| p.flags[1] = 4, ..Spec::default() }), VALUE);
}

#[test]
fn the_pcr_list_must_be_exactly_the_version_4_one() {
    assert_eq!(
        refused(Spec {
            doc: |p| {
                p.pcrs.pop();
            },
            ..Spec::default()
        }),
        VALUE
    );
    let more = |p: &mut crate::build::doc::Parts| p.pcrs.push((15, [0; 32]));
    assert_eq!(refused(Spec { doc: more, ..Spec::default() }), VALUE);
    assert_eq!(refused(Spec { doc: |p| p.pcrs.swap(3, 4), ..Spec::default() }), VALUE);
    assert_eq!(refused(Spec { doc: |p| p.pcrs[7].0 = 15, ..Spec::default() }), VALUE);
}
