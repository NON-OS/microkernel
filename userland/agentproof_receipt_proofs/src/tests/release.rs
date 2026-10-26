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

//! The release verdict, apart from verification: a receipt verifies by every
//! check and then says whether it is from a release, by what admitted each
//! capsule PCR 14 records; `--release` alone refuses one that is not.

use agentproof_receipt::cli::{answer, answer_in};
use agentproof_receipt::verify;
use agentproof_receipt::verify::Release;
use nsh_attest::log::Admitted;

use super::cli::written;
use crate::build::tcg_capsule::{path_only_tag, DEVELOPER, PATH_ONLY};
use crate::build::tcg_log::Log;
use crate::fixture::Fixture;

fn with(kinds: &[u8]) -> Fixture {
    let mut f = Fixture::new(b"strict");
    let mut log = Log::standard();
    for (i, k) in kinds.iter().enumerate() {
        let verifier = if *k == PATH_ONLY { path_only_tag() } else { [0xC0 + i as u8; 32] };
        log.capsule([i as u8 + 1; 32], verifier, *k);
    }
    f.spec.log = log;
    f
}

#[test]
fn every_capsule_under_the_vendors_root_is_a_release() {
    let mut f = Fixture::new(b"strict");
    let (bytes, t) = (f.bytes(), f.trust());
    assert_eq!(verify(&bytes, &t).unwrap().is_release, Release::Release);
    let a = answer_in(&bytes, &written(&t), true);
    assert_eq!((a.code, a.out.iter().any(|l| l == "kind     release")), (0, true));
}

#[test]
fn a_development_image_verifies_and_is_not_a_release() {
    let mut f = with(&[PATH_ONLY, PATH_ONLY, 1, PATH_ONLY, DEVELOPER]);
    let (bytes, t) = (f.bytes(), f.trust());
    let v = verify(&bytes, &t).unwrap();
    let not = vec![(Admitted::PathOnly, 3), (Admitted::Developer, 1)];
    assert_eq!(v.is_release, Release::NotRelease(not));
    let said = "not a release: 3 path-only capsules, 1 developer capsules";
    assert_eq!(v.is_release.said(), said);
    let a = answer(&bytes, &written(&t));
    assert_eq!(a.code, 0, "verified");
    assert!(a.out.contains(&format!("kind     {said}")));
    let a = answer_in(&bytes, &written(&t), true);
    assert_eq!((a.code, a.err[0].as_str()), (1, &*format!("agentproof-receipt: {said}")));
}

#[test]
fn a_boot_that_admitted_no_capsule_is_not_a_release() {
    let mut f = with(&[]);
    let (bytes, t) = (f.bytes(), f.trust());
    let v = verify(&bytes, &t).unwrap();
    assert_eq!(v.is_release.said(), "not a release: PCR 14 records no capsule");
    assert_eq!(answer_in(&bytes, &written(&t), true).code, 1);
}
