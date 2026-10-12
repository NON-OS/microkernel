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

//! Pairing the user's answer does not complete: digits typed wrong, and
//! no answer at all, which the SMP timer ends (Vol 3, Part H, 3.4).

use super::answer_link_tests::{peer, SHOWN};
use crate::fixture::{air, link};
use crate::link::{Answer, End};
use nonos_bt_smp::method::IoCap;

#[test]
fn digits_typed_wrong_end_pairing() {
    let (mut host, ours, mut env) = peer(IoCap::DisplayOnly);
    env.user.answers.push_back(Answer::Passkey(SHOWN ^ 1));
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::GaveUp("pairing refused by the peer"), "its confirm check failed");
    assert!(s.bond.is_none());
    assert_eq!(air(&mut host).ltk, None, "no key was ever used");
}

#[test]
fn a_user_who_never_answers_runs_out_the_pairing_timer() {
    let (mut host, ours, mut env) = peer(IoCap::DisplayYesNo);
    env.user.keep_waiting = true;
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::GaveUp("pairing failed"));
    assert!(s.bond.is_none());
    assert!(env.now >= 30_000, "gave up early, at {} ms", env.now);
}

#[test]
fn numbers_the_user_says_differ_end_pairing() {
    let (mut host, ours, mut env) = peer(IoCap::DisplayYesNo);
    env.user.answers.push_back(Answer::Same(false));
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::GaveUp("pairing failed"), "declined here, not by the peer");
    assert!(s.bond.is_none());
}
