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

//! The DMC seal and the DC states kept off without it.

use crate::dmc::{admit, admit_in, blob};
use crate::dmc_parse::parse;
use crate::engine::dc::{dc_off, DC_STATE_EN};
use crate::engine::error::Error;
use crate::family::Family;
use crate::model::{Gen9, Polls};

const BLOB: &[u8] = b"a display microcontroller program";

fn sealed_line(name: &str, bytes: &[u8]) -> Vec<u8> {
    let hex: String = nonos_hash::sha256(bytes).iter().map(|b| format!("{b:02x}")).collect();
    format!("# test seal\n{name} {hex}\n").into_bytes()
}

#[test]
fn nothing_is_sealed_so_nothing_loads() {
    assert_eq!(admit(Family::Skylake, BLOB), None);
    assert_eq!(admit(Family::TigerLake, b""), None);
}

#[test]
fn a_sealed_blob_loads_for_its_families_only() {
    let src = sealed_line("kbl_dmc", BLOB);
    let seal = parse(Box::leak(src.into_boxed_slice()));
    assert_eq!(seal.len, 1);
    assert_eq!(admit_in(&seal, Family::CoffeeLake, BLOB), Some(BLOB));
    assert_eq!(admit_in(&seal, Family::KabyLake, BLOB), Some(BLOB));
    assert_eq!(admit_in(&seal, Family::Skylake, BLOB), None, "skl_dmc, not kbl_dmc");
    assert_eq!(admit_in(&seal, Family::KabyLake, b"a display microcontroller progran"), None);
    assert_eq!(admit_in(&seal, Family::Broadwell, BLOB), None, "Gen8 has no DMC");
}

#[test]
fn every_family_but_broadwell_names_a_blob() {
    assert_eq!(blob(Family::Broadwell), None);
    assert_eq!(blob(Family::RaptorLakeP), blob(Family::AlderLakeP));
    assert_eq!(blob(Family::IceLake), Some("icl_dmc"));
}

#[test]
fn dc5_and_dc6_are_switched_off_and_a_stuck_bit_times_out() {
    let mut m = Gen9::new(64);
    m.set(DC_STATE_EN, 0x2 | (1 << 8));
    dc_off(&mut m, &Polls::new(5)).unwrap();
    assert_eq!(m.get(DC_STATE_EN), 1 << 8, "other bits kept");
    let writes = m.writes.len();
    dc_off(&mut m, &Polls::new(5)).unwrap();
    assert_eq!(m.writes.len(), writes, "already off: nothing written");
    m.set(DC_STATE_EN, 0x1);
    m.dead = true;
    assert_eq!(dc_off(&mut m, &Polls::new(5)), Err(Error::Timeout));
}
