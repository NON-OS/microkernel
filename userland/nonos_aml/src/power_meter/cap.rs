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

//! Setting the hardware cap with _SHL, the meter's one write here: only
//! when _PMC says a hardware enforced cap exists and is configurable, and
//! only within its bounds. _SHL answers 0 done, 1 out of range, 2 failed
//! (recalled from ACPI 6.5, 10.4.7: unverified). The capsule calls this
//! only with its switch on.

use super::pmc::Pmc;
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, integer, Eval};
use alloc::vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapError {
    /// No enforced cap, or one firmware does not let the OS set.
    NotConfigurable,
    OutOfRange,
    Failed,
}

pub fn set_cap(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    dev: Id,
    pmc: &Pmc,
    mw: u32,
) -> Result<(), CapError> {
    if pmc.capabilities & 4 == 0 || !pmc.cap_configurable {
        return Err(CapError::NotConfigurable);
    }
    let (lo, hi) = (pmc.min_cap_mw.unwrap_or(0), pmc.max_cap_mw.unwrap_or(u32::MAX));
    if !(lo..=hi).contains(&mw) {
        return Err(CapError::OutOfRange);
    }
    let r = child(aml, eval, dev, *b"_SHL", vec![Object::Integer(u64::from(mw))]);
    match r.and_then(|r| r.ok()).as_ref().and_then(integer) {
        Some(0) => Ok(()),
        Some(1) => Err(CapError::OutOfRange),
        _ => Err(CapError::Failed),
    }
}
