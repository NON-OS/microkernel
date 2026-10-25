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

//! Step 3, the binding: `extraData` is the client's own `qualifying_data`
//! over the document's challenge, registry root and DMA posture.

use nsh_attest::Refusal;

use super::refused;
use crate::build::Spec;

#[test]
fn extra_data_that_is_not_the_clients_binding_is_refused() {
    assert_eq!(refused(Spec { quote: |q| q.extra[0] ^= 1, ..Spec::default() }), Refusal::Binding);
    let short = |q: &mut crate::build::attest::Quote| {
        q.extra.pop();
    };
    assert_eq!(refused(Spec { quote: short, ..Spec::default() }), Refusal::Binding);
    assert_eq!(refused(Spec { quote: |q| q.extra.clear(), ..Spec::default() }), Refusal::Binding);
}

#[test]
fn a_dma_posture_other_than_the_one_bound_is_refused() {
    // The document says what the quote did not sign.
    for bound in [(1, true, 4, 0), (1, true, 0, 2)] {
        let s = Spec { bound_dma: Some(bound), ..Spec::default() };
        assert_eq!(refused(s), Refusal::Binding, "{bound:?}");
    }
    let s = Spec { root: Some([0x11; 32]), doc: |p| p.root = [0x12; 32], ..Spec::default() };
    assert_eq!(refused(s), Refusal::Binding);
}
