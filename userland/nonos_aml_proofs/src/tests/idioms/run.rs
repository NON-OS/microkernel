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

//! One evaluation with its costs: the result, the steps it took and the
//! time it spent in Sleep, Stall and waits.

use crate::exec::{ExecLimits, Fault, Machine};
use crate::object::Object;
use crate::rig::Rig;

pub struct Ran {
    pub result: Result<Object, Fault>,
    pub steps: u64,
    pub time_us: u64,
}

pub fn run(r: &mut Rig, path: &str, args: Vec<Object>, limits: ExecLimits) -> Ran {
    let id = crate::consts::at(&r.aml, path).unwrap_or_else(|| panic!("{path} exists"));
    let mut m = Machine::new(&mut r.aml, &mut r.probe, limits);
    let result = m.evaluate(&mut r.aml, id, args);
    Ran { result, steps: m.steps(), time_us: m.time_us }
}

pub fn ok(r: &mut Rig, path: &str) -> (u64, Ran) {
    let ran = run(r, path, vec![], ExecLimits::DEFAULT);
    let v = match &ran.result {
        Ok(o) => crate::run::num(o),
        Err(f) => panic!("{path}: {f:?}"),
    };
    (v, ran)
}
