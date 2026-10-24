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

//! Step 1, the version: only 4 is read. Version 3 is refused even when
//! everything else about it is genuine, because it quotes too little.

use nsh_attest::consts::MAX_DOC;
use nsh_attest::{Doc, FormatFault, Refusal};

use super::{passes, refused};
use crate::build::Spec;

#[test]
fn version_4_parses_and_places_every_field() {
    let b = Spec::default().build();
    let d = Doc::parse(&b.doc).expect("parses");
    assert_eq!((d.capsule_count, d.registry_complete), (3, true));
    assert_eq!((d.iommu_vendor, d.iommu_enforcing, d.unconfined_grants), (1, true, 0));
    assert_eq!(d.pcr(9), Some(&[0x09; 32]));
    assert_eq!(d.pcr(14), Some(&[0x14; 32]));
    assert_eq!(d.pcr(3), None);
    assert_eq!(d.ak_public, &crate::build::ak::Key::new(1).public());
    assert!(!d.log.is_empty() && !d.attest.is_empty() && !d.signature.is_empty());
}

#[test]
fn a_genuine_version_3_document_is_refused() {
    assert_eq!(refused(Spec { version: 3, ..Spec::default() }), Refusal::Version(3));
}

#[test]
fn a_version_3_number_on_a_version_4_body_is_refused() {
    let s = Spec { doc: |p| p.version = 3, ..Spec::default() };
    assert_eq!(refused(s), Refusal::Version(3));
}

#[test]
fn unknown_versions_are_refused() {
    for v in [0, 1, 2, 5, 0x0400_0000, u32::MAX] {
        let s = Spec { version: v, ..Spec::default() };
        assert_eq!(refused(s), Refusal::Version(v), "version {v}");
    }
}

#[test]
fn a_wrong_magic_is_refused() {
    let s = Spec { bytes: |d| d[7] = b'S', ..Spec::default() };
    assert_eq!(refused(s), Refusal::Format(FormatFault::Magic));
}

#[test]
fn a_document_over_the_cap_is_refused_before_it_is_read() {
    let mut b = Spec::default().build();
    b.doc.resize(MAX_DOC + 1, 0);
    assert_eq!(Doc::parse(&b.doc), Err(Refusal::Format(FormatFault::Oversize)));
    passes(Spec::default());
}
