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

//! What the three functions say. Function 5 answers an integer: 0, the
//! operating system must keep the configuration firmware made at boot; 1,
//! it may ignore it. Function 9 a package of five durations in
//! microseconds: after a conventional reset, after the link comes up, after
//! a function-level reset, from D3hot to D0, and after VF enable (recalled,
//! unverified). Anything of another shape is no answer.

use super::call::ask;
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{integer, Eval};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Readiness {
    pub reset_us: u32,
    pub link_up_us: u32,
    pub flr_us: u32,
    pub d3hot_to_d0_us: u32,
    pub vf_enable_us: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HostFacts {
    /// Some(true): the boot configuration must be kept.
    pub preserve_boot_config: Option<bool>,
    pub readiness: Option<Readiness>,
}

fn readiness(o: &Object) -> Option<Readiness> {
    let Object::Package(p) = o else { return None };
    let e = p.borrow();
    let us = |i: usize| e.get(i).and_then(integer).and_then(|v| u32::try_from(v).ok());
    let (reset_us, link_up_us, flr_us) = (us(0)?, us(1)?, us(2)?);
    let (d3hot_to_d0_us, vf_enable_us) = (us(3)?, us(4)?);
    Some(Readiness { reset_us, link_up_us, flr_us, d3hot_to_d0_us, vf_enable_us })
}

pub fn host_facts(aml: &mut Aml, eval: &mut Eval<'_>, host: Id) -> HostFacts {
    let preserve = match ask(aml, eval, host, 5) {
        Some(Object::Integer(0)) => Some(true),
        Some(Object::Integer(1)) => Some(false),
        _ => None,
    };
    let readiness = ask(aml, eval, host, 9).as_ref().and_then(readiness);
    HostFacts { preserve_boot_config: preserve, readiness }
}
