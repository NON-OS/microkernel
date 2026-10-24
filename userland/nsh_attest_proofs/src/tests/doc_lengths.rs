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

//! Step 1, the lengths: each within its cap and its floor, nothing left over
//! and nothing cut short.

use nsh_attest::consts::{MAX_ATTEST, MAX_LOG, MAX_SIG};
use nsh_attest::{Doc, FormatFault, Refusal};

use super::refused;
use crate::build::Spec;

const VALUE: Refusal = Refusal::Format(FormatFault::Value);
const OVERSIZE: Refusal = Refusal::Format(FormatFault::Oversize);

#[test]
fn lengths_over_their_caps_are_refused() {
    let log = |p: &mut crate::build::doc::Parts| p.log.resize(MAX_LOG + 1, 0);
    assert_eq!(refused(Spec { doc: log, ..Spec::default() }), OVERSIZE);
    let at = |p: &mut crate::build::doc::Parts| p.attest.resize(MAX_ATTEST + 1, 0);
    assert_eq!(refused(Spec { doc: at, ..Spec::default() }), OVERSIZE);
    let sig = |p: &mut crate::build::doc::Parts| p.sig.resize(MAX_SIG + 1, 0);
    assert_eq!(refused(Spec { doc: sig, ..Spec::default() }), OVERSIZE);
    assert_eq!(refused(Spec { doc: |p| p.ak.push(4), ..Spec::default() }), OVERSIZE);
}

#[test]
fn lengths_under_their_floors_are_refused() {
    assert_eq!(refused(Spec { doc: |p| p.attest.clear(), ..Spec::default() }), VALUE);
    assert_eq!(refused(Spec { doc: |p| p.sig.clear(), ..Spec::default() }), VALUE);
    assert_eq!(
        refused(Spec {
            doc: |p| {
                p.ak.pop();
            },
            ..Spec::default()
        }),
        VALUE
    );
}

#[test]
fn a_trailing_byte_is_refused() {
    let s = Spec { bytes: |d| d.push(0), ..Spec::default() };
    assert_eq!(refused(s), Refusal::Format(FormatFault::Trailing));
}

#[test]
fn a_document_cut_anywhere_is_refused_as_short() {
    let doc = Spec::default().build().doc;
    for cut in 0..doc.len() {
        assert_eq!(Doc::parse(&doc[..cut]), Err(Refusal::Format(FormatFault::Short)), "{cut}");
    }
}
