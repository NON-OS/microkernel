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

//! _LPI's rules: a level is taken whole or not at all, because the level
//! below names its states by position.

use super::build::reg;
use super::lpi_rig::{lpi, state};
use crate::cpu::*;
use crate::enc::*;
use crate::object::Object;
use crate::run::eval;

fn value(v: &[u8]) -> Object {
    eval(&ret(v), vec![]).unwrap()
}

#[test]
fn a_level_is_taken_whole() {
    let good = state(1, 2, 1, 0, reg(0x7F, 32, 0, 3, 7), reg(0, 64, 0, 4, 0x9000));
    let l = parse_lpi(&value(&lpi(5, core::slice::from_ref(&good)))).unwrap();
    assert_eq!((l.level, l.states[0].wake_latency_us, l.states[0].flags), (5, 2, LPI_ENABLED));
    assert_eq!(
        l.states[0].entry,
        LpiEntry::Register(Register { space: 0x7F, width: 32, access: 3, address: 7, offset: 0 })
    );
    assert_eq!((l.states[0].residency.unwrap().address, l.states[0].usage), (0x9000, None));
    assert_eq!(l.states[0].name, b"S");
    let null = state(1, 2, 1, 0, reg(0, 0, 0, 0, 0), int(0));
    assert_eq!(parse_lpi(&value(&lpi(0, &[good.clone(), null]))), None, "null entry");
    let wide = state(1 << 32, 2, 1, 0, int(1), int(0));
    assert_eq!(parse_lpi(&value(&lpi(0, &[wide]))), None, "residency past u32");
    let lying = package(4, &[int(0), int(0), int(2), good.clone()]);
    assert_eq!(parse_lpi(&value(&lying)), None, "count says two, holds one");
    let many: Vec<Vec<u8>> = (0..=MAX_LPI_STATES).map(|_| good.clone()).collect();
    assert_eq!(parse_lpi(&value(&lpi(0, &many))), None);
}
