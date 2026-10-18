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

//! Entering S3 in the order ACPI 6.5 gives (7.4 and 16.1.3): \_S3 read
//! first, so a machine without it runs nothing; \_TTS(3), which tells the
//! platform where the system is going; the policy's wake devices armed;
//! \_PTS(3); \_SI._SST(3), the sleeping indicator; then, with the Global
//! Lock free, the waking vector set, WAK_STS cleared and SLP_TYP/SLP_EN
//! written. Whatever happened, the way back (`resume`) runs, so a sleep
//! the hardware refused leaves the platform as it was.

use super::hardware::SleepHardware;
use super::outcome::{Outcome, SuspendError};
use super::resume::{resume, sst, INDICATOR_SLEEPING};
use super::wake::arm;
use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::Eval;
use crate::sleep::{notify_state, sleep_types, SleepTypes};

pub const S3: u8 = 3;

fn enter(hw: &mut impl SleepHardware, types: SleepTypes, vector: u64) -> Result<(), SuspendError> {
    if hw.global_lock_held() {
        return Err(SuspendError::LockHeld);
    }
    hw.set_waking_vector(vector).map_err(SuspendError::Hardware)?;
    hw.clear_wake_status().map_err(SuspendError::Hardware)?;
    hw.sleep(types).map_err(SuspendError::Hardware)
}

/// `wake` is the owner's policy: the devices allowed to wake the system.
pub fn suspend_s3(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    hw: &mut impl SleepHardware,
    wake: &[Id],
    vector: u64,
) -> Outcome {
    let Some(types) = sleep_types(aml, eval, S3) else {
        return Outcome::refused(SuspendError::NoS3);
    };
    let mut out = Outcome { s0: sleep_types(aml, eval, 0), ..Outcome::default() };
    let _ = notify_state(aml, eval, *b"_TTS", S3);
    let mut armed = alloc::vec::Vec::new();
    for &dev in wake {
        match arm(aml, eval, hw, dev, S3) {
            Ok(a) => armed.push(a),
            Err(skip) => out.skipped.push((dev, skip)),
        }
    }
    let _ = notify_state(aml, eval, *b"_PTS", S3);
    sst(aml, eval, INDICATOR_SLEEPING);
    match enter(hw, types, vector) {
        Ok(()) => out.entered = true,
        Err(e) => out.error = Some(e),
    }
    out.armed = armed.iter().map(|a| a.dev).collect();
    out.wak = resume(aml, eval, hw, &armed);
    out
}
