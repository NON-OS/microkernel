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

//! The rest of the proof of the key: the subject is H(the keys in
//! `USER_AUTH`), and the user's signatures over this transcript both
//! verify. Every failure of that part is told as `USER_SIG`.

use super::login_key::login;
use crate::build::grant::GrantB;
use crate::build::keys::{key, K};
use crate::build::run::Setup;
use crate::build::user_auth::UserAuthB;
use crate::build::Spoil;
use nsh_roles::Refusal;
use nsh_wire::{Grant, Reason, UserAuth};

#[test]
fn the_subject_must_be_the_users_keys() {
    let g = GrantB { subject: key(K::OtherUser).subject(), ..GrantB::default() };
    assert_eq!(login(g), Err(Refusal::Subject));
    let ua = UserAuthB { user: K::OtherUser, signer: K::OtherUser, ..ua() };
    assert_eq!(Setup::default().login_ua(&ua).err(), Some(Refusal::Subject));
}

fn ua() -> UserAuthB {
    UserAuthB::new(GrantB::default().bytes())
}

#[test]
fn both_halves_of_the_users_signature_must_verify_over_this_transcript() {
    let s = Setup::default();
    assert_eq!(
        s.login_ua(&UserAuthB { signer: K::OtherUser, ..ua() }).err(),
        Some(Refusal::UserSig)
    );
    assert_eq!(s.login_ua(&UserAuthB { th: [0x7B; 64], ..ua() }).err(), Some(Refusal::UserSig));
    for spoil in [Spoil::Ed, Spoil::Ml] {
        assert_eq!(s.login_ua(&UserAuthB { spoil, ..ua() }).err(), Some(Refusal::UserSig));
    }
}

#[test]
fn every_failure_of_the_first_part_is_told_as_user_sig() {
    for r in [Refusal::Issuer, Refusal::IssuerSig, Refusal::Subject, Refusal::UserSig] {
        assert_eq!(r.reason(), Reason::UserSig);
    }
}

#[test]
fn an_escalation_grant_is_not_a_login() {
    let e = GrantB::escalation().bytes();
    let b = GrantB::default().bytes();
    let ua_bytes = UserAuthB::new(b).bytes();
    let mut ua = UserAuth::parse(&ua_bytes).expect("parses");
    ua.grant = Grant::parse(&e).expect("parses");
    assert_eq!(Setup::default().login_auth(&ua), Err(Refusal::Kind));
}
