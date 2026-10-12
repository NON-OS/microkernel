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

//! What the device says, read-only: _GCP's capabilities, _GRT's time, and
//! for each timer (0 on AC, 1 on DC) _TIV's value in seconds, _TIP's
//! expired-timer policy in seconds, and _GWS's wake status. A method that
//! is absent, fails, or answers out of shape reads as None.

use super::time::RealTime;
use crate::load::Aml;
use crate::namespace::{Id, Seg};
use crate::object::Object;
use crate::platform::{child, integer, Eval};
use alloc::vec;
use alloc::vec::Vec;

pub const AC: u64 = 0;
pub const DC: u64 = 1;
/// _TIV's and _TIP's answer for a timer that is not running.
pub const DISABLED: u64 = 0xFFFF_FFFF;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Timer {
    pub value: Option<u64>,
    pub policy: Option<u64>,
    pub wake_status: Option<u64>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TimeAlarm {
    pub capabilities: Option<u64>,
    pub now: Option<RealTime>,
    pub ac: Timer,
    pub dc: Timer,
}

fn int(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id, seg: Seg, args: Vec<Object>) -> Option<u64> {
    child(aml, eval, dev, seg, args)?.ok().as_ref().and_then(integer)
}

fn timer(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id, which: u64) -> Timer {
    let mut ask = |seg| int(aml, eval, dev, seg, vec![Object::Integer(which)]);
    Timer { value: ask(*b"_TIV"), policy: ask(*b"_TIP"), wake_status: ask(*b"_GWS") }
}

pub fn read(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> TimeAlarm {
    let capabilities = int(aml, eval, dev, *b"_GCP", Vec::new());
    let now = match child(aml, eval, dev, *b"_GRT", Vec::new()) {
        Some(Ok(Object::Buffer(b))) => RealTime::decode(&b.borrow()),
        _ => None,
    };
    let (ac, dc) = (timer(aml, eval, dev, AC), timer(aml, eval, dev, DC));
    TimeAlarm { capabilities, now, ac, dc }
}
