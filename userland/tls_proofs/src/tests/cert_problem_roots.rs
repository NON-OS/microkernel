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

//! The reason given when a chain stops short of a root, or has none to read.

use super::super::cert_message_vectors::message;
use super::INSIDE;
use crate::cert_problem::{cert_problem, CertProblem};
use crate::chain_walk::verify_chain;
use crate::example_leaf::EXAMPLE_LEAF;
use crate::fixtures::certs::GATEWAY;

#[test]
fn a_chain_cut_short_of_a_known_root_names_the_issuer() {
    let body = message(&[EXAMPLE_LEAF]);
    assert_eq!(verify_chain(&body, b"example.com", INSIDE), Ok(false));
    assert_eq!(cert_problem(&body, b"example.com", INSIDE), Some(CertProblem::UnknownIssuer));
}
#[test]
fn a_private_root_is_an_unknown_issuer() {
    let body = message(&GATEWAY);
    let now = 20261001000000;
    assert_eq!(verify_chain(&body, b"nonos.software", now), Ok(false));
    assert_eq!(cert_problem(&body, b"nonos.software", now), Some(CertProblem::UnknownIssuer));
}
#[test]
fn an_unreadable_list_says_so() {
    assert_eq!(cert_problem(&message(&[]), b"example.com", INSIDE), Some(CertProblem::Unreadable));
    assert_eq!(cert_problem(&[], b"example.com", INSIDE), Some(CertProblem::Unreadable));
}

/* The local test chain's throwaway root, which only a devnet build knows. */
#[path = "dev_root.rs"]
mod dev_root;
#[path = "dev_root_trust.rs"]
mod dev_root_trust;
