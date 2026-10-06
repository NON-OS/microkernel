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

//! The `MkBootAttest` record: the kernel's encoder against libc's parser.

use crate::kernel_boot_record::{encode, Answer, Enrolled, RECORD_LEN};
use crate::libc_boot_record::{parse_boot_attest, AdmittedLoader, BootAttest, BOOT_ATTEST_LEN};

const E: Enrolled = Enrolled { measurement: [0x33; 32], root: [0x44; 32], epoch: 5 };
const SEEN: AdmittedLoader = AdmittedLoader { measurement: [0x33; 32], root: [0x44; 32], epoch: 5 };

fn read(a: Answer) -> Option<BootAttest> {
    parse_boot_attest(&encode(&a))
}

#[test]
fn the_two_sides_agree_on_the_length() {
    assert_eq!(RECORD_LEN, BOOT_ATTEST_LEN);
}

#[test]
fn every_verdict_round_trips_and_keeps_its_own_state() {
    assert_eq!(read(Answer::NotYet), Some(BootAttest::NotYet));
    assert_eq!(read(Answer::Measured(E)), Some(BootAttest::Measured(SEEN)));
    assert_eq!(read(Answer::SelfReported(E)), Some(BootAttest::SelfReported(SEEN)));
    assert_eq!(read(Answer::Refused(403)), Some(BootAttest::Refused(403)));
    assert_eq!(read(Answer::NoEvidence), Some(BootAttest::NoEvidence));
}

/* A self-reported pass must never read back as a measured one. */
#[test]
fn self_reported_and_measured_differ_in_the_state_byte_alone() {
    let (m, s) = (encode(&Answer::Measured(E)), encode(&Answer::SelfReported(E)));
    assert_eq!((m[1], s[1]), (1, 2));
    assert_eq!(m[2..], s[2..]);
}

#[test]
fn a_verdict_that_admits_nothing_carries_no_values() {
    for a in [Answer::NotYet, Answer::Refused(1), Answer::NoEvidence] {
        assert!(encode(&a)[8..].iter().all(|&b| b == 0));
    }
}

#[test]
fn a_malformed_record_is_refused() {
    let base = encode(&Answer::Measured(E));
    for (at, v) in [(0usize, 2u8), (1, 5), (2, 1), (3, 1), (4, 1)] {
        let mut r = base;
        r[at] = v;
        assert!(parse_boot_attest(&r).is_none(), "byte {at} = {v:#x} accepted");
    }
    let mut r = encode(&Answer::NoEvidence);
    r[60] = 1;
    assert!(parse_boot_attest(&r).is_none());
    let mut r = encode(&Answer::Refused(7));
    r[4..8].copy_from_slice(&[0; 4]);
    assert!(parse_boot_attest(&r).is_none());
    assert!(parse_boot_attest(&base[..RECORD_LEN - 1]).is_none());
}
