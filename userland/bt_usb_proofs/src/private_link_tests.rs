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

//! A link made from the private address: pairing runs with it as the
//! initiator's address (Vol 3, Part H, 2.2.7), and this host's identity
//! goes to a peer that asks for it (3.6.4, 3.6.5).

use crate::air::Act;
use crate::fixture::{air, keyboard, link_as};
use crate::link::Private;
use crate::private_tests::IRK;
use nonos_bt_bond::LocalIrk;
use nonos_bt_hogp::report::Input;
use nonos_bt_smp::pdu::{keys, le, IDENTITY_ADDRESS_INFORMATION, IDENTITY_INFORMATION};

#[test]
fn a_link_from_the_private_address_gives_the_peer_this_hosts_identity() {
    let (mut host, ours, mut env) = keyboard();
    air(&mut host).smp.ours.init_keys = keys::ID;
    air(&mut host).script.extend([Act::Notify(11, vec![0, 0, 4, 0, 0, 0, 0, 0]), Act::Drop(0x13)]);
    let mut private = Private::new(LocalIrk::new(IRK).unwrap(), ours);
    let us = private.refresh(&mut host, &mut env).unwrap();
    let (_, s) = link_as(&mut host, &us, &mut env).unwrap();
    assert!(s.bond.expect("not paired").authenticated);
    let seen = &air(&mut host).seen;
    assert_eq!(seen.own_types, [1]);
    let address = [&[IDENTITY_ADDRESS_INFORMATION, 0][..], &ours.0].concat();
    assert_eq!(seen.identity, [[&[IDENTITY_INFORMATION][..], &le(&IRK)].concat(), address]);
    assert_eq!(env.input, [Input::KeyDown(4), Input::KeyUp(4)]);
}

#[test]
fn a_peer_that_asks_no_identity_is_given_none() {
    let (mut host, ours, mut env) = keyboard();
    air(&mut host).script.push_back(Act::Drop(0x13));
    let mut private = Private::new(LocalIrk::new(IRK).unwrap(), ours);
    let us = private.refresh(&mut host, &mut env).unwrap();
    let (_, s) = link_as(&mut host, &us, &mut env).unwrap();
    assert!(s.bond.is_some());
    assert!(air(&mut host).seen.identity.is_empty());
}
