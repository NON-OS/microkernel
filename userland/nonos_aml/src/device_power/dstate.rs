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

//! Moving a device between D0 and D3 (ACPI 6.5, 7.3): for D0, the power
//! resources _PR0 lists first, then _PS0; for D3, _PS3 first, then those
//! resources given back. A device with neither list nor methods is always
//! on, and both moves succeed without touching it.

use super::method::run;
use super::pool::PowerPool;
use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::{listed, Eval};

pub fn to_d0(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    pool: &mut PowerPool,
    dev: Id,
) -> Result<(), Fault> {
    let res = listed(aml, eval, dev, *b"_PR0");
    pool.acquire(aml, eval, &res)?;
    run(aml, eval, dev, *b"_PS0")
}

pub fn to_d3(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    pool: &mut PowerPool,
    dev: Id,
) -> Result<(), Fault> {
    let ps3 = run(aml, eval, dev, *b"_PS3");
    let res = listed(aml, eval, dev, *b"_PR0");
    let off = pool.release(aml, eval, &res);
    ps3.and(off)
}
