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

//! Directed advertising the host must not take: to an address made under
//! another key, while it has no key, and from a device it holds no bond
//! with however the target resolves.

use super::fixture::{bonded, old_rpa};
use crate::fixture::{air, keyboard};
use crate::fixture_link::plain;
use crate::link::{find_hid, Private};
use crate::private_tests::IRK;
use nonos_bt_bond::LocalIrk;

#[test]
fn an_address_made_under_another_key_is_not_ours() {
    let (mut host, mut env, us) = bonded(true);
    air(&mut host).seen.directed_to = Some(old_rpa(b"another-hosts-ky"));
    assert_eq!(find_hid(&mut host, &mut env, &us), Ok(None));
}

#[test]
fn a_host_with_no_key_takes_no_directed_report() {
    let (mut host, mut env, _) = bonded(true);
    air(&mut host).seen.directed_to = Some(old_rpa(&IRK));
    assert_eq!(find_hid(&mut host, &mut env, &plain()), Ok(None));
}

#[test]
fn a_stranger_aimed_at_our_address_is_passed_over() {
    let (mut host, ours, mut env) = keyboard();
    air(&mut host).cfg.le_features = 0x80;
    host.bring_up().unwrap();
    air(&mut host).seen.directed_to = Some(old_rpa(&IRK));
    let mut private = Private::new(LocalIrk::new(IRK).unwrap(), ours);
    let me = private.refresh(&mut host, &mut env).unwrap();
    assert_eq!(find_hid(&mut host, &mut env, &me), Ok(None), "no bond, however aimed");
}
