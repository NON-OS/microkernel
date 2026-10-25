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

//! A login's first part, the proof of the key: the issuer is a role
//! authority whose signatures both verify, the subject is H(the user's
//! keys), and the user's signatures over the transcript both verify. Every
//! failure here is told as `USER_SIG`.

use crate::build::grant::GrantB;
use crate::build::keys::{key, K};
use crate::build::run::Setup;
use crate::build::{Spoil, BASE_ID, HOUR, NOW};
use nonos_hash::sha512;
use nsh_roles::{CapSet, Refusal};
use nsh_wire::Grant;

pub fn login(g: GrantB) -> Result<(), Refusal> {
    Setup::default().login(&g).map(|_| ())
}

#[test]
fn a_good_login_is_admitted_with_what_its_grant_names() {
    let g = GrantB::default();
    let l = Setup::default().login(&g).expect("admitted");
    assert_eq!(l.grant_id, BASE_ID);
    assert_eq!((l.user.as_slice(), l.role.as_slice()), (&b"ek"[..], &b"operator"[..]));
    assert_eq!(l.caps.names(), ["CoreExec", "IPC", "Memory"]);
    assert_eq!(l.ends_ms, NOW + 4 * HOUR);
    assert_eq!(l.subject, key(K::User).subject());
    let bytes = g.bytes();
    let issued = &bytes[..g.signed().len() + 64 + 3309];
    assert_eq!(l.hash, sha512(issued));
    assert_eq!(l.hash, nsh_roles::grant_hash(&Grant::parse(&bytes).expect("parses")));
    assert!(!l.record_digest && l.caps != CapSet::default());
}

#[test]
fn a_session_ends_at_the_roles_longest_session_if_that_is_first() {
    let g = GrantB { not_after_ms: NOW + 10 * HOUR, ..GrantB::default() };
    assert_eq!(Setup::default().login(&g).expect("admitted").ends_ms, NOW + 8 * HOUR);
}

#[test]
fn the_issuer_must_be_a_role_authority() {
    for k in [K::Stranger, K::Appr1, K::User] {
        let g = GrantB { issuer: key(k).id(), signer: k, ..GrantB::default() };
        assert_eq!(login(g), Err(Refusal::Issuer), "{k:?}");
    }
    let by_b = GrantB { issuer: key(K::AuthB).id(), signer: K::AuthB, ..GrantB::default() };
    assert_eq!(login(by_b), Ok(()));
}

#[test]
fn both_halves_of_the_issuers_signature_must_verify() {
    assert_eq!(login(GrantB { signer: K::AuthB, ..GrantB::default() }), Err(Refusal::IssuerSig));
    for spoil in [Spoil::Ed, Spoil::Ml] {
        assert_eq!(login(GrantB { spoil, ..GrantB::default() }), Err(Refusal::IssuerSig));
    }
}
