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

//! Addresses from drawn keys and drawn prand, as a capsule makes them:
//! each one resolvable, resolving under its own key and no other.

use crate::sim::Rng;
use nonos_bt_smp::pairing::Random;
use nonos_bt_smp::privacy::{prand, resolves, rpa};
use nonos_bt_wire::addr::RandomKind;

#[test]
fn every_drawn_address_resolves_under_its_key_only() {
    let mut rng = Rng(0x0dd_ba11);
    let (mut key, mut other) = ([0u8; 16], [0u8; 16]);
    rng.fill(&mut key);
    rng.fill(&mut other);
    let mut made = 0;
    for _ in 0..1000 {
        let mut draw = [0u8; 3];
        rng.fill(&mut draw);
        let Some(p) = prand(draw) else { continue };
        let a = rpa(&key, &p);
        assert_eq!(a.random_kind(), Ok(RandomKind::Resolvable));
        assert!(resolves(&key, &a));
        assert!(!resolves(&other, &a));
        made += 1;
    }
    assert!(made > 990);
}
