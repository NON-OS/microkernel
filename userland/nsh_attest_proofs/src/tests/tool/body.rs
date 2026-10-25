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

//! A machine enrolled by the tool passes, with its boot applications read
//! from its own log: every application that is not a release loader, each
//! once, in the order it ran.

use nsh_wire::Enrol;

use super::{check, inputs, key, request};
use crate::build::tcg::EV_APP;
use crate::build::tcg_event::digest;
use crate::build::Spec;

#[test]
fn a_certificate_the_tool_makes_passes_the_client_s_whole_check() {
    let b = Spec::default().build();
    let (keys, loaders) = inputs();
    let body = nsh_enrol::body(&request(&b, &keys, &loaders)).expect("a body");
    let cert = nsh_enrol::sign(&body, key(), 0).expect("signed");
    let e = Enrol::parse(&cert).expect("parses");
    assert_eq!(e.boot_applications().collect::<Vec<_>>(), [&digest(b"boot manager")]);
    assert_eq!(check(&b, cert), Ok(()));
}

#[test]
fn every_boot_application_is_enrolled_once_in_the_order_it_ran() {
    let mut s = Spec::default();
    s.log.ext(4, EV_APP, digest(b"first stage")).ext(4, EV_APP, digest(b"boot manager"));
    let b = s.build();
    let (keys, loaders) = inputs();
    let body = nsh_enrol::body(&request(&b, &keys, &loaders)).expect("a body");
    let cert = nsh_enrol::sign(&body, key(), 0).expect("signed");
    let apps: Vec<[u8; 32]> =
        Enrol::parse(&cert).expect("parses").boot_applications().copied().collect();
    assert_eq!(apps, [digest(b"boot manager"), digest(b"first stage")]);
    assert_eq!(check(&b, cert), Ok(()));
}

#[test]
fn the_selection_keeps_order_drops_loaders_and_repeats() {
    let (a, l, c) = ([1; 32], [2; 32], [3; 32]);
    assert_eq!(nsh_enrol::applications(&[a, l, c, a], &[l], &[a, c]), Ok(vec![a, c]));
    assert_eq!(nsh_enrol::applications(&[l], &[l], &[]), Ok(vec![]));
    assert_eq!(nsh_enrol::applications(&[a, c], &[l], &[a, c]), Err(nsh_enrol::Error::NoLoader));
    let zero = nsh_enrol::applications(&[[0; 32], l], &[l], &[[0; 32]]);
    assert_eq!(zero, Err(nsh_enrol::Error::Value("boot application digest")));
    let l = [0xEE; 32];
    let many: Vec<[u8; 32]> = (1..=9).map(|i| [i; 32]).chain([l]).collect();
    assert_eq!(
        nsh_enrol::applications(&many, &[l], &many),
        Err(nsh_enrol::Error::BootApplications(9))
    );
    assert_eq!(nsh_enrol::applications(&many[1..], &[l], &many).map(|a| a.len()), Ok(8));
}
