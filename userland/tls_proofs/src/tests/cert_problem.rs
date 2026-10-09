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

//! The reason a refused certificate list is given, read after the refusal.

use super::cert_message_vectors::message;
use crate::cert_problem::{cert_problem, CertProblem};
use crate::cert_window::cert_window;
use crate::chain_walk::verify_chain;
use crate::example_ca::EXAMPLE_CA;
use crate::example_leaf::EXAMPLE_LEAF;
use crate::fixtures::certs::GATEWAY;

/* The leaf was served on 2026-09-20 and runs from 2026-07-29 to 2026-10-27. */
const INSIDE: u64 = 20260920000000;

#[test]
fn the_window_reads_the_same_fields_the_check_compares() {
    let (from, until) = cert_window(EXAMPLE_LEAF).expect("window");
    assert_eq!(from / 1_000_000, 20260729);
    assert_eq!(until / 1_000_000, 20261027);
    assert!(cert_window(&EXAMPLE_LEAF[..40]).is_none());
}
#[test]
fn an_issuer_whose_own_issuer_is_not_a_root_is_unknown() {
    /*
     * The name and both windows are in order. The intermediate was issued by
     * a root this store does not carry, and the walk refuses the pair for
     * exactly that, so that is the reason given.
     */
    let body = message(&[EXAMPLE_LEAF, EXAMPLE_CA]);
    assert!(!verify_chain(&body, b"example.com", INSIDE));
    assert_eq!(cert_problem(&body, b"example.com", INSIDE), Some(CertProblem::UnknownIssuer));
}
#[test]
fn another_name_is_named_as_a_mismatch() {
    let body = message(&[EXAMPLE_LEAF, EXAMPLE_CA]);
    assert!(!verify_chain(&body, b"example.org", INSIDE));
    assert_eq!(cert_problem(&body, b"example.org", INSIDE), Some(CertProblem::NameMismatch));
}
#[test]
fn either_side_of_the_window_is_told_apart() {
    let body = message(&[EXAMPLE_LEAF, EXAMPLE_CA]);
    let (from, until) = cert_window(EXAMPLE_LEAF).expect("window");
    for (now, want) in [(from - 1, CertProblem::NotYetValid), (until + 1, CertProblem::Expired)] {
        assert!(!verify_chain(&body, b"example.com", now));
        assert_eq!(cert_problem(&body, b"example.com", now), Some(want));
    }
}
#[test]
fn a_chain_cut_short_of_a_known_root_names_the_issuer() {
    let body = message(&[EXAMPLE_LEAF]);
    assert!(!verify_chain(&body, b"example.com", INSIDE));
    assert_eq!(cert_problem(&body, b"example.com", INSIDE), Some(CertProblem::UnknownIssuer));
}
#[test]
fn a_private_root_is_an_unknown_issuer() {
    let body = message(&GATEWAY);
    let now = 20261001000000;
    assert!(!verify_chain(&body, b"nonos.software", now));
    assert_eq!(cert_problem(&body, b"nonos.software", now), Some(CertProblem::UnknownIssuer));
}
#[test]
fn an_unreadable_list_says_so() {
    assert_eq!(cert_problem(&message(&[]), b"example.com", INSIDE), Some(CertProblem::Unreadable));
    assert_eq!(cert_problem(&[], b"example.com", INSIDE), Some(CertProblem::Unreadable));
}
