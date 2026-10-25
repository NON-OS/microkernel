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

//! Step 2: the document answers this session's challenge, which depends on
//! `TH(SERVER_HELLO)` and names the host keys.

use nsh_attest::Refusal;

use super::refused;
use crate::build::Spec;

#[test]
fn a_document_made_for_another_session_is_refused() {
    let b = Spec::default().build();
    let mut other = Spec::default().build();
    other.th = [0x78; 64];
    other.doc = b.doc;
    assert_eq!(other.check(), Err(Refusal::Challenge));
}

#[test]
fn a_document_over_another_challenge_is_refused() {
    let s = Spec { challenge: Some([0x5A; 32]), ..Spec::default() };
    assert_eq!(refused(s), Refusal::Challenge);
}

#[test]
fn a_document_naming_other_host_keys_is_refused() {
    // Another key pair signs HOST_PROOF; the document names the host's.
    let mut b = Spec::default().build();
    b.host_key = crate::build::keys::STRANGER;
    assert_eq!(b.check(), Err(Refusal::Challenge));
}
