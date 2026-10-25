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

//! A wake device's enable method, both ways, and disarming it: GPE off,
//! _DSW(0, 0, 0) or _PSW(0), its _PRW power resources off newest first.

use super::hardware::SleepHardware;
use super::wake::Armed;
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, Eval};
use alloc::vec;
use alloc::vec::Vec;

pub fn enable(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id, on: bool, state: u8) {
    let (on, state) = (u64::from(on), u64::from(state));
    let dsw = vec![Object::Integer(on), Object::Integer(state), Object::Integer(on * 3)];
    if child(aml, eval, dev, *b"_DSW", dsw).is_none() {
        let _ = child(aml, eval, dev, *b"_PSW", vec![Object::Integer(on)]);
    }
}

pub fn disarm(aml: &mut Aml, eval: &mut Eval<'_>, hw: &mut impl SleepHardware, a: &Armed) {
    if let Some(g) = a.gpe {
        let _ = hw.arm_gpe(g, false);
    }
    enable(aml, eval, a.dev, false, 0);
    for r in a.resources.iter().rev() {
        let _ = child(aml, eval, *r, *b"_OFF", Vec::new());
    }
}
