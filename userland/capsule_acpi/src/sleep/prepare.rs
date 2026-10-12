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

//! PREPARE_SLEEP (5, state): a shutdown is coming. \_TTS(5) and \_PTS(5)
//! run, \_S5 is read again, and its values go to the kernel, so the PM1
//! write that follows is the one firmware asked for (ACPI 6.5, 16.1).
//! A failed \_TTS or \_PTS is logged and the shutdown goes on.

use crate::boot::{evaluator, Acpi};
use crate::server::protocol::{EINVAL, ENOENT};
use crate::sys::acpi_sleep::{acpi_sleep, SOFT_OFF};
use crate::sys::log::Line;
use nonos_aml::exec::Fault;
use nonos_aml::sleep::prepare;

fn say(what: &[u8], r: &Option<Result<(), Fault>>) {
    match r {
        None => Line::new(what).str(b" absent").end(),
        Some(Ok(())) => Line::new(what).str(b"(5) ran").end(),
        Some(Err(f)) => Line::new(what).str(b"(5) faulted at ").hex(u64::from(f.at)).end(),
    }
}

pub fn prepare_sleep(acpi: &mut Acpi, state: Option<u8>) -> i32 {
    // S3 and suspend-to-idle answer only with the owner's switch on.
    match state {
        Some(3) if crate::suspend::SUSPEND => return crate::suspend::s3(acpi),
        Some(0) if crate::suspend::SUSPEND => return crate::suspend::idle(acpi),
        _ => {}
    }
    if state != Some(SOFT_OFF) {
        return EINVAL;
    }
    let p = prepare(&mut acpi.aml, &mut evaluator(&mut acpi.host), SOFT_OFF);
    say(b"_TTS", &p.tts);
    say(b"_PTS", &p.pts);
    let Some(t) = p.types else {
        Line::new(b"\\_S5 unusable at shutdown").end();
        return ENOENT;
    };
    match acpi_sleep(SOFT_OFF, t) {
        Ok(()) => 0,
        Err(e) => e as i32,
    }
}
