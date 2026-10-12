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

//! The firmware's half of entering a sleeping state (ACPI 6.5, 16.1 and
//! 7.4): \_TTS(x) first, which tells the platform where the system is
//! going; then \_PTS(x), which prepares it (on many machines it is what
//! tells the embedded controller to cut power); then \_Sx, evaluated last
//! because firmware may compute it. Writing SLP_TYP and SLP_EN is the
//! kernel's.

use super::sx::{parse_sx, sx_seg, SleepTypes};
use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::{Seg, ROOT};
use crate::object::Object;
use crate::platform::Eval;
use alloc::vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prepared {
    /// None when the method is absent.
    pub tts: Option<Result<(), Fault>>,
    pub pts: Option<Result<(), Fault>>,
    /// None when \_Sx is absent, faulted, or is not a sleep package: the
    /// state cannot be entered.
    pub types: Option<SleepTypes>,
}

pub(crate) fn notify_state(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    seg: Seg,
    x: u8,
) -> Option<Result<(), Fault>> {
    let id = aml.ns.child(ROOT, seg)?;
    Some(eval(aml, id, vec![Object::Integer(u64::from(x))]).map(|_| ()))
}

/// Reads \_Sx alone, with nothing run: for knowing at start what the state
/// will take.
pub fn sleep_types(aml: &mut Aml, eval: &mut Eval<'_>, x: u8) -> Option<SleepTypes> {
    let id = aml.ns.child(ROOT, sx_seg(x))?;
    eval(aml, id, alloc::vec::Vec::new()).ok().as_ref().and_then(parse_sx)
}

/// Runs \_TTS(x) and \_PTS(x), then reads \_Sx. A failed \_TTS or \_PTS is
/// reported and does not stop the rest: the state is still entered.
pub fn prepare(aml: &mut Aml, eval: &mut Eval<'_>, x: u8) -> Prepared {
    let tts = notify_state(aml, eval, *b"_TTS", x);
    let pts = notify_state(aml, eval, *b"_PTS", x);
    Prepared { tts, pts, types: sleep_types(aml, eval, x) }
}
