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
use nonos_libc::mk_device_release;

use super::attempt::{one, Attempt};
use super::driver::Started;
use super::{claim, config};
use crate::controller::verdict::{combine, Verdict};
use crate::discover::{Found, Survey};
use crate::error::HdaResult;
use crate::protocol::OutputStatus;

/// One bring-up attempt over every controller discovery found, chipset
/// controllers first. The first that can play is kept; the others' claims
/// are given back as each attempt ends. A graphics card's controller that
/// fails is passed over, since it never carries the speakers; any other
/// failure fails the attempt, so the bounded retry schedule runs again.
pub fn run(survey: &Survey, verdict_seen: &mut Option<Verdict>) -> HdaResult<Started> {
    let mut last_err = None;
    for dev in survey.controllers() {
        match one(*dev) {
            Ok(Attempt::Playing(d)) => return Ok(Started::Playing(d)),
            Ok(Attempt::Silent(v)) => {
                *verdict_seen = Some(verdict_seen.map_or(v, |w| combine(w, v)));
            }
            Err(_) if dev.graphics() => {}
            Err(e) => last_err = Some(e),
        }
    }
    if let Some(e) = last_err {
        return Err(e);
    }
    Ok(Started::Silent(OutputStatus::silent(machine_verdict(survey, *verdict_seen) as u32)))
}

/// What the machine as a whole is, once no controller plays.
pub fn machine_verdict(survey: &Survey, seen: Option<Verdict>) -> Verdict {
    let v = seen.unwrap_or(Verdict::NoCodec);
    if survey.amd_acp {
        combine(v, Verdict::AmdAcp)
    } else {
        v
    }
}

/// The claim on `dev`, with memory decoding on and bus mastering off: the
/// engines a dead owner left armed are stopped before it comes on
/// (`attempt`). A claim whose command word is refused is given straight
/// back: nothing else has been taken with it yet.
pub(super) fn claimed(dev: &Found) -> HdaResult<u64> {
    let epoch = claim::claim(dev.device_id)?;
    if let Err(e) = config::enable_memory(dev.device_id, epoch) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    Ok(epoch)
}
