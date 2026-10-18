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

//! A keyboard bonded over a first link, then heard again on a controller
//! brought up with or without the extended filter policies.

use crate::bond_tests::{press, static_identity};
use crate::env::TestEnv;
use crate::fixture::{air, keyboard, link, Stack};
use crate::link::us::Us;
use crate::link::Private;
use crate::private_tests::IRK;
use nonos_bt_bond::LocalIrk;
use nonos_bt_smp::privacy::rpa;

/// A private address of ours the controller no longer uses.
pub fn old_rpa(irk: &[u8; 16]) -> [u8; 6] {
    rpa(irk, &[0x47, 0x12, 0x34]).0
}

/// A keyboard bonded over a first link, a controller with or without the
/// feature brought up again, and this host on a private address.
pub fn bonded(extended: bool) -> (Stack, TestEnv, Us) {
    let (mut host, ours, mut env) = keyboard();
    static_identity(&mut host);
    press(&mut host);
    link(&mut host, &ours, &mut env).unwrap();
    air(&mut host).cfg.le_features = if extended { 0x80 } else { 0 };
    host.bring_up().unwrap();
    let mut private = Private::new(LocalIrk::new(IRK).unwrap(), ours);
    let us = private.refresh(&mut host, &mut env).unwrap();
    (host, env, us)
}
