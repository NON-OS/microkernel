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

//! The client reads the challenge from the document itself, at the offset
//! the attester names. An attester that does not hold the document to the
//! challenge, and so cannot help echoing the one it was given, still has
//! to point at the document's own bytes: a document for another challenge
//! is refused wherever it points.

use super::host_run;
use crate::kit::attest::{doc, TestAttester, DOC_LEN};
use crate::kit::pair::{config, Pair};
use crate::kit::proof::ProofSpec;
use nsh_client::{HostRefusal, Refusal};

fn lax_at(at: usize) -> TestAttester {
    TestAttester { lax: true, challenge_at: Some(at), ..TestAttester::default() }
}

#[test]
fn a_document_for_another_challenge_is_refused_wherever_the_attester_points() {
    let spec = ProofSpec { doc: Some(doc(&[0x99; 32])), ..ProofSpec::good() };
    for at in [4, 0, 36, DOC_LEN - 32, DOC_LEN - 31, DOC_LEN, usize::MAX - 8, usize::MAX] {
        let (r, p) = host_run(&spec, config(), lax_at(at));
        assert_eq!(r, Refusal::Host(HostRefusal::Challenge), "offset {at}");
        assert!(p.client.attester().said_ok.get(), "the attester said yes");
    }
}

#[test]
fn the_right_offset_into_the_right_document_passes_and_no_other() {
    let mut p = Pair::with(config(), lax_at(4));
    p.to_proof();
    let hp = p.proof(&ProofSpec::good());
    assert_eq!(p.feed(&hp), Ok(()));
    let (r, _) = host_run(&ProofSpec::good(), config(), lax_at(5));
    assert_eq!(r, Refusal::Host(HostRefusal::Challenge));
}
