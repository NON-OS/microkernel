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

use super::gate_build::{booted, signed, TRAILER};
use super::record_build::{record, verifier};
use crate::gate::{measured, BootError};
use crate::record::RecordError;

#[test]
fn a_stale_record_and_a_bad_signature_are_refused() {
    let (log, pcr) = booted(&[]);
    let stale = measured(&log, &pcr, 6, &signed(5), TRAILER, verifier);
    assert_eq!(stale, Err(BootError::Record(RecordError::Stale)));
    let mut forged = signed(5);
    forged[40] ^= 1;
    let bad = measured(&log, &pcr, 5, &forged, TRAILER, verifier);
    assert_eq!(bad, Err(BootError::Record(RecordError::BadSignature)));
}

#[test]
fn a_signed_record_for_another_root_does_not_admit_the_loader() {
    let (log, pcr) = booted(&[]);
    let other = record([9u8; 32], 5);
    assert_eq!(measured(&log, &pcr, 5, &other, TRAILER, verifier), Err(BootError::Path));
}
