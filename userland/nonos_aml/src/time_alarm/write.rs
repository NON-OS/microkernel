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

//! The two writes: _SRT sets firmware's clock, _STV(timer, seconds) arms a
//! wake alarm on AC or DC. Both answer 0 for done; anything else is
//! firmware's refusal. Whether either may be called at all is the
//! caller's switch, not this module's.

use super::read::{AC, DC};
use super::time::RealTime;
use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, integer, Eval};
use alloc::vec;

#[derive(Debug, PartialEq, Eq)]
pub enum WriteError {
    /// A time out of range, or a timer that is neither AC nor DC.
    Invalid,
    /// The device has no such method.
    Absent,
    /// The method answered something other than 0.
    Refused(u64),
    Fault(Fault),
}

fn done(r: Option<Result<Object, Fault>>) -> Result<(), WriteError> {
    let v = r.ok_or(WriteError::Absent)?.map_err(WriteError::Fault)?;
    match integer(&v) {
        Some(0) => Ok(()),
        other => Err(WriteError::Refused(other.unwrap_or(u64::MAX))),
    }
}

pub fn set_time(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    dev: Id,
    t: &RealTime,
) -> Result<(), WriteError> {
    let b = t.encode().ok_or(WriteError::Invalid)?;
    done(child(aml, eval, dev, *b"_SRT", vec![Object::buffer(b.to_vec())]))
}

pub fn set_alarm(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    dev: Id,
    timer: u64,
    seconds: u32,
) -> Result<(), WriteError> {
    if timer != AC && timer != DC {
        return Err(WriteError::Invalid);
    }
    let args = vec![Object::Integer(timer), Object::Integer(u64::from(seconds))];
    done(child(aml, eval, dev, *b"_STV", args))
}
