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

//! Devices for runtime wake, all on GPE 0x6D with PRW1 (on says 0xC1, off
//! 0xC0): XHC wakes from D3hot (`_S0W` 3) and has `_DSW`; TPD has no
//! `_S0W` and only `_PSW`; LAN wakes from D3cold. NOP has no `_PRW`, BLK a
//! block-form event, BAD an `_S0W` of 7. Each method says its arguments.

use super::rig::say;
use crate::consts::at;
use crate::device_power::GpeSwitch;
use crate::enc::*;
use crate::load::Aml;

fn prw(event: Vec<u8>) -> Vec<u8> {
    name_obj("_PRW", &package(3, &[event, int(3), name("\\PRW1")]))
}

fn tell(n: u8) -> Vec<u8> {
    (0..n).flat_map(|i| store(&arg(i), &op(0x5B31))).collect()
}

pub fn src() -> Vec<u8> {
    let on_off = cat(&[&method("_ON_", 0, &say(0xC1)), &method("_OFF", 0, &say(0xC0))]);
    let s0w = |n: u64| method("_S0W", 0, &ret(&int(n)));
    let block = package(2, &[name("\\_SB.XHC"), int(1)]);
    cat(&[
        &power_res("\\PRW1", 0, 0, &on_off),
        &device("\\_SB.XHC", &cat(&[&prw(int(0x6D)), &s0w(3), &method("_DSW", 3, &tell(3))])),
        &device("\\_SB.TPD", &cat(&[&prw(int(0x6D)), &method("_PSW", 1, &tell(1))])),
        &device("\\_SB.LAN", &cat(&[&prw(int(0x6D)), &s0w(4), &method("_DSW", 3, &tell(3))])),
        &device("\\_SB.NOP", &s0w(3)),
        &device("\\_SB.BLK", &prw(block)),
        &device("\\_SB.BAD", &cat(&[&prw(int(0x6D)), &s0w(7)])),
    ])
}

pub fn ids(aml: &Aml) -> [u32; 7] {
    ["XHC", "TPD", "LAN", "NOP", "BLK", "BAD"]
        .map(|d| at(aml, &format!("\\_SB.{d}")).unwrap())
        .into_iter()
        .chain([at(aml, "\\PRW1").unwrap()])
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

/// Records each switch of a wake event; refuses when `fail` is set.
#[derive(Default)]
pub struct Switches {
    pub log: Vec<(u16, bool)>,
    pub fail: bool,
}

impl GpeSwitch for Switches {
    fn switch(&mut self, gpe: u16, on: bool) -> bool {
        self.log.push((gpe, on));
        !self.fail
    }
}
