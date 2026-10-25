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

//! Step 5, the release: PCR 9 picks one accepted release, and PCR 8 must
//! be that release's. Zeros are refused. PCR 14 is no release's value: it is
//! replayed from the log, and a value the log does not give is refused.

use nsh_attest::Refusal;

use super::{passes, refused};

mod capsules;
use crate::build::parts::release;
use crate::build::Spec;

fn with(pcr: u8, v: [u8; 32]) -> Refusal {
    refused(Spec { pcr: vec![(pcr, v)], ..Spec::default() })
}

#[test]
fn a_kernel_no_release_names_is_refused() {
    assert_eq!(with(9, [0x99; 32]), Refusal::Pcr9);
}

#[test]
fn a_loader_not_the_releases_or_a_pcr_14_the_log_does_not_give_is_refused() {
    assert_eq!(with(8, [0x88; 32]), Refusal::Pcr8);
    assert_eq!(with(14, [0x41; 32]), Refusal::Replay(14));
}

#[test]
fn a_pcr_left_at_zero_is_refused() {
    assert_eq!(with(9, [0; 32]), Refusal::Pcr9);
    assert_eq!(with(8, [0; 32]), Refusal::Pcr8);
    assert_eq!(with(14, [0; 32]), Refusal::Replay(14));
}

#[test]
fn any_accepted_release_may_be_the_one_running() {
    let mut next = release();
    (next.version, next.pcr8, next.pcr9) = ("0.9.5".into(), [0x18; 32], [0x19; 32]);
    let s = Spec { releases: vec![release(), next], ..Spec::default() };
    let (v, pcr9) = s.build().check().expect("the older release still runs");
    assert_eq!((v.as_str(), pcr9), ("0.9.4", [0x09; 32]));
    let s = Spec { pcr: vec![(8, [0x18; 32]), (9, [0x19; 32])], ..s };
    assert_eq!(s.build().check().expect("the newer").1, [0x19; 32]);
    passes(Spec::default());
}

#[test]
fn one_releases_kernel_with_anothers_loader_is_refused() {
    let mut next = release();
    (next.pcr8, next.pcr9) = ([0x18; 32], [0x19; 32]);
    let s = Spec { releases: vec![release(), next], pcr: vec![(9, [0x19; 32])], ..Spec::default() };
    assert_eq!(refused(s), Refusal::Pcr8);
}
