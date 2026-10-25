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

//! Step 7: the entries fold to the bound root, the registry is complete,
//! the stated count is the count sent, and `nsh.front` is the release's.

use nsh_attest::steps::fold;
use nsh_attest::{Evidence, EvidenceError, Refusal};
use nsh_crypto::Transcript;

use super::refused;
use crate::build::parts::entry;
use crate::build::Spec;

#[test]
fn the_fold_is_the_receipt_readers_fold() {
    let e = Spec::default().entries.concat();
    assert_eq!(fold(&e), attest_receipt::fold_root(&e));
    assert_eq!(fold(&[]), attest_receipt::fold_root(&[]));
}

#[test]
fn entries_that_are_not_the_signed_set_are_refused() {
    let mut b = Spec::default().build();
    b.entries[40] ^= 1;
    assert_eq!(b.check(), Err(Refusal::Registry));
    b.entries[40] ^= 1;
    b.entries.extend(entry(9, [9; 32], 0));
    assert_eq!(b.check(), Err(Refusal::Registry));
}

#[test]
fn a_ragged_empty_or_oversize_set_is_refused() {
    // nsh_wire holds HOST_PROOF's entries to whole records, 1 to 1024.
    let mut b = Spec::default().build();
    b.entries.push(0);
    let no = |b: &crate::build::Built| {
        let r = Evidence::verify(&b.proof(), &Transcript::default()).map(|_| ());
        assert!(matches!(r, Err(EvidenceError::Wire(_))));
    };
    no(&b);
    no(&Spec { entries: vec![], ..Spec::default() }.build());
    let many: Vec<_> = (0..1025).map(|p| entry(p, [0xF0; 32], 0)).collect();
    no(&Spec { entries: many, ..Spec::default() }.build());
}

#[test]
fn an_incomplete_registry_is_refused() {
    let s = Spec { doc: |p| p.flags[0] = 0, ..Spec::default() };
    assert_eq!(refused(s), Refusal::RegistryIncomplete);
}

#[test]
fn a_stated_count_other_than_the_sent_one_is_refused() {
    assert_eq!(refused(Spec { doc: |p| p.count = 4, ..Spec::default() }), Refusal::CapsuleCount);
    assert_eq!(refused(Spec { doc: |p| p.count = 2, ..Spec::default() }), Refusal::CapsuleCount);
}
