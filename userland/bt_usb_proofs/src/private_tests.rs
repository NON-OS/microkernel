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

//! This host's resolvable private address: set between scans, renewed
//! every TGAP(private_addr_int) (Vol 3, Part C, Appendix A), and resolved
//! under the IRK it gives peers in pairing.

use crate::fixture::{air, keyboard, Stack};
use crate::link::Private;
use nonos_bt_bond::{LocalIrk, ROTATE_MS};
use nonos_bt_smp::pairing::Identity;
use nonos_bt_smp::privacy::resolves;
use nonos_bt_wire::addr::AddrType;

pub const IRK: [u8; 16] = *b"this-hosts-irk!!";

fn addresses_set(host: &mut Stack) -> usize {
    air(host).commands.iter().filter(|c| c[..2] == [0x05, 0x20]).count()
}

#[test]
fn the_first_address_is_private_and_resolves_under_this_hosts_irk() {
    let (mut host, ours, mut env) = keyboard();
    let mut private = Private::new(LocalIrk::new(IRK).unwrap(), ours);
    let us = private.refresh(&mut host, &mut env).unwrap();
    assert_eq!(us.kind, AddrType::Random);
    assert!(resolves(&IRK, &us.addr));
    assert_eq!(air(&mut host).seen.random, Some(us.addr.0));
    assert_eq!(us.identity, Some(Identity { irk: IRK, addr_type: 0, addr: ours.0 }));
}

#[test]
fn the_address_changes_only_when_its_fifteen_minutes_are_up() {
    let (mut host, ours, mut env) = keyboard();
    let mut private = Private::new(LocalIrk::new(IRK).unwrap(), ours);
    let first = private.refresh(&mut host, &mut env).unwrap().addr;
    env.now += ROTATE_MS - 10;
    assert_eq!(private.refresh(&mut host, &mut env).unwrap().addr, first);
    assert_eq!(addresses_set(&mut host), 1);
    env.now += 10;
    let second = private.refresh(&mut host, &mut env).unwrap().addr;
    assert_ne!(second, first);
    assert!(resolves(&IRK, &second));
    assert_eq!(addresses_set(&mut host), 2);
}

#[test]
fn coming_back_on_after_off_draws_a_fresh_address() {
    let (mut host, ours, mut env) = keyboard();
    let mut private = Private::new(LocalIrk::new(IRK).unwrap(), ours);
    let first = private.refresh(&mut host, &mut env).unwrap().addr;
    env.now += 10; // nowhere near the fifteen minutes: it would keep `first`
    private.resume(); // the radio went off and is coming back on (E-72)
    let next = private.refresh(&mut host, &mut env).unwrap().addr;
    assert_ne!(next, first, "a fresh address, not the one it went off under");
    assert!(resolves(&IRK, &next));
    assert_eq!(addresses_set(&mut host), 2);
}
