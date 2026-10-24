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

//! The firmware PCRs, 0, 1, 2 and 7, must be the ones the machine was
//! enrolled with, whether the enrolment or the quote differs.

use super::host_refusal;
use crate::kit::attest::doc_with;
use crate::kit::proof::ProofSpec;
use crate::kit::{AK, PCRS};
use nsh_client::{HostRefusal, Refusal};

#[test]
fn the_firmware_pcrs_must_be_the_enrolled_ones() {
    for i in 0..4 {
        let mut spec = ProofSpec::good();
        spec.enrol.pcrs[i][31] ^= 1;
        assert_eq!(host_refusal(&spec), Refusal::Host(HostRefusal::FirmwarePcrs), "PCR {i}");
    }
    let mut quoted = PCRS;
    quoted[3][0] ^= 1;
    let mut p = crate::kit::pair::Pair::good();
    p.to_proof();
    let spec = ProofSpec {
        doc: Some(doc_with(&ProofSpec::good().challenge(&p.server), &AK, &quoted)),
        ..ProofSpec::good()
    };
    let hp = p.proof(&spec);
    assert_eq!(p.feed(&hp), Err(Refusal::Host(HostRefusal::FirmwarePcrs)));
}
