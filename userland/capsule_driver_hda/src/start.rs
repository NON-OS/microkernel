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

//! From the device survey to serving: playing, saying why not, or giving up.

use nonos_libc::{mk_exit, start_driver};

use crate::error::{reason, HdaError};
use crate::protocol::OutputStatus;
use crate::setup::{Line, Started};
use crate::{discover, server, setup};

const DRIVER: &[u8] = b"driver.hda";

/// Without any audio hardware the driver says so and leaves (`EXIT_ABSENT`)
/// before claiming anything. Controllers that are there are brought up on the
/// shared bounded schedule, each failed attempt releasing what it took;
/// running out marks the last failure with its numbered code and leaves with
/// `EXIT_GAVE_UP`. A machine whose audio this driver cannot play (a DSP-only
/// Intel laptop, an AMD ACP, only HDMI) is not a failure: the driver gives
/// its claims back and stays to say why, so the audio server and the
/// applications can tell the user in plain words instead of going silent.
/// An Intel SST engine on the bus starts the verdict at "needs SOF", so a
/// Broadwell's HDMI-only controller beside it, or one that fails, cannot
/// hide where the speakers are.
pub fn start() -> ! {
    let survey = discover::survey();
    if let Some(dev) = survey.intel_sst {
        Line::new("[HDA] intel sst dsp 8086:").hex(dev as u32, 4).s(" found, needs SOF").emit();
    }
    let dsp = survey.amd_acp || survey.intel_sst.is_some();
    if survey.is_empty() && dsp {
        let v = setup::machine_verdict(&survey, survey.bus_verdict());
        server::run_status(OutputStatus::silent(v as u32));
    }
    let found = if survey.is_empty() { None } else { Some(survey) };
    let mut last: Option<HdaError> = None;
    let mut seen = survey.bus_verdict();
    let started = start_driver(DRIVER, found, |s| {
        setup::run(s, &mut seen).map_err(|e| {
            last = Some(e);
            reason(e)
        })
    });
    match started {
        Ok(Started::Playing(driver)) => server::run(driver),
        Ok(Started::Silent(status)) => server::run_status(status),
        Err(code) => {
            if let Some(e) = last {
                setup::mark_setup_fail(e);
            }
            if let Some(v) = survey.bus_verdict() {
                server::run_status(OutputStatus::silent(v as u32));
            }
            mk_exit(code)
        }
    }
}
