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
//! A whole receipt verifies, and says what it carries.
//! The doc's checks: it is signed for this receipt, by the pinned AK, from a
//! trusted release, on a machine whose IOMMU enforced, and carries nothing
//! its quote does not sign.

use agentproof_receipt::{verify, Refusal};

use crate::fixture::Fixture;

fn refused(f: &mut Fixture) -> &'static str {
    let (bytes, trust) = (f.bytes(), f.trust());
    verify(&bytes, &trust).unwrap_err().name()
}

#[test]
fn the_doc_must_be_signed_for_this_receipt_by_the_pinned_ak_and_release() {
    let mut f = Fixture::new(b"strict");
    let (mut bytes, trust) = (f.bytes(), f.trust());
    bytes[30] ^= 1; // inside the record, after the doc was asked for it
    assert!(matches!(verify(&bytes, &trust), Err(Refusal::Challenge)));
    bytes[30] ^= 1;
    let mut t = trust.clone();
    t.ak[0] ^= 1;
    assert_eq!(verify(&bytes, &t).unwrap_err().name(), "ak");
    let mut t = trust.clone();
    t.releases[0].pcr14[0] ^= 1;
    assert_eq!(verify(&bytes, &t).unwrap_err().name(), "release");
    let mut t = trust.clone();
    t.releases[0].pcr9[0] ^= 1;
    assert_eq!(verify(&bytes, &t).unwrap_err().name(), "release");
}

#[test]
fn the_machine_must_have_had_its_iommu_enforcing_and_its_registry_whole() {
    // Not enforcing; a grant around it; no IOMMU at all; a device whose
    // domain reaches all of memory.
    for dma in [(1, false, 0, 0), (1, true, 1, 0), (0, true, 0, 0), (1, true, 0, 1)] {
        let mut f = Fixture::new(b"strict");
        f.spec.dma = dma;
        assert_eq!(refused(&mut f), "dma");
    }
}

#[test]
fn a_doc_carrying_its_unsigned_count_or_log_is_refused() {
    use agentproof_receipt::napr::{challenge, write};
    let mut f = Fixture::new(b"strict");
    let trust = f.trust();
    let rec = f.record.encode();
    let bound = write::bound(&rec, &f.box_word, &f.policy, &f.launcher);
    f.spec.challenge = Some(challenge(&bound));
    let kernel_doc = f.spec.doc_bytes();
    let whole = write::finish(bound, &kernel_doc);
    assert!(matches!(verify(&whole, &trust), Err(Refusal::Unsigned)), "as the kernel wrote it");
}
