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

//! The way back from S3, or from an S3 that never happened (ACPI 6.5, 7.4.5
//! and 16.1.4): \_SI._SST(2), waking; \_WAK(3), whose answer says whether
//! the wake went as asked; the wake devices disarmed, newest first;
//! \_TTS(0), back to working; \_SI._SST(1).

use super::hardware::SleepHardware;
use super::s3::S3;
use super::unwake::disarm;
use super::wake::Armed;
use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::ROOT;
use crate::object::Object;
use crate::platform::{child, Eval};
use crate::sleep::notify_state;
use alloc::vec;

/// _SST's values (ACPI 6.5, 9.1.1): working, waking, sleeping.
pub const INDICATOR_WORKING: u8 = 1;
pub const INDICATOR_WAKING: u8 = 2;
pub const INDICATOR_SLEEPING: u8 = 3;

/// \_SI._SST(x), when the platform has a system indicator.
pub fn sst(aml: &mut Aml, eval: &mut Eval<'_>, x: u8) {
    if let Some(si) = aml.ns.child(ROOT, *b"_SI_") {
        let _ = child(aml, eval, si, *b"_SST", vec![Object::Integer(u64::from(x))]);
    }
}

pub fn resume(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    hw: &mut impl SleepHardware,
    armed: &[Armed],
) -> Option<Result<Object, Fault>> {
    sst(aml, eval, INDICATOR_WAKING);
    let wak =
        aml.ns.child(ROOT, *b"_WAK").map(|id| eval(aml, id, vec![Object::Integer(u64::from(S3))]));
    for a in armed.iter().rev() {
        disarm(aml, eval, hw, a);
    }
    let _ = notify_state(aml, eval, *b"_TTS", 0);
    sst(aml, eval, INDICATOR_WORKING);
    wak
}
