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

use super::gate_build::{booted, measurement, root, signed, LOADER, TRAILER};
use super::log_build::digest;
use super::record_build::verifier;
use crate::gate::{measured, self_reported, BootError};

#[test]
fn the_enrolled_loader_measured_by_the_firmware_is_admitted() {
    let (log, pcr) = booted(&[]);
    let a = measured(&log, &pcr, 5, &signed(5), TRAILER, verifier).expect("admitted");
    assert_eq!((a.measurement, a.root, a.epoch), (measurement(), root(), 5));
}

#[test]
fn a_log_that_does_not_replay_to_the_live_pcr_is_refused() {
    let (log, mut pcr) = booted(&[]);
    pcr[0] ^= 1;
    assert_eq!(measured(&log, &pcr, 5, &signed(5), TRAILER, verifier), Err(BootError::PcrMismatch));
}

#[test]
fn an_application_started_after_the_loader_is_what_gets_checked_and_fails() {
    let (log, pcr) = booted(&[digest(42)]);
    assert_eq!(measured(&log, &pcr, 5, &signed(5), TRAILER, verifier), Err(BootError::Path));
}

#[test]
fn a_flipped_proof_byte_is_refused_with_the_verifier_code() {
    let (log, pcr) = booted(&[]);
    let mut t = TRAILER.to_vec();
    let last = t.len() - 1;
    t[last] ^= 1;
    let r = measured(&log, &pcr, 5, &signed(5), &t, verifier);
    assert!(matches!(r, Err(BootError::Proof(_))), "{r:?}");
    assert!(r.err().is_some_and(|e| e.code() > 400));
}

#[test]
fn without_a_tpm_the_loader_file_is_checked_and_a_changed_byte_is_refused() {
    let a = self_reported(LOADER, &signed(5), TRAILER, verifier).expect("admitted");
    assert_eq!(a.measurement, measurement());
    let mut changed = LOADER.to_vec();
    changed[0x300] ^= 1;
    let r = self_reported(&changed, &signed(5), TRAILER, verifier);
    assert_eq!(r, Err(BootError::Path));
}
