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

//! Taking the platform from legacy mode into ACPI mode (ACPI 6.5, 4.8.2.1
//! and 16.3.1): OSPM writes ACPI_ENABLE to SMI_CMD and waits for firmware to
//! set SCI_EN. Until then the SCI never fires and firmware's SMI handler
//! still owns the events.

use super::error::EventsError;
use super::fadt::Fadt;
use super::io::EventIo;
use super::pm1_cnt::Pm1Cnt;

/// Between looks at SCI_EN.
const POLL_MS: u64 = 10;

/// Ok(true) when this call switched the mode, Ok(false) when it was already
/// on. `wait_ms` bounds the wait; firmware that never answers is an error,
/// not a hang.
pub fn enter_acpi_mode(f: &Fadt, io: &mut impl EventIo, wait_ms: u64) -> Result<bool, EventsError> {
    let cnt = Pm1Cnt { a: f.pm1a_cnt.ok_or(EventsError::NoPm1)?, b: f.pm1b_cnt };
    if cnt.sci_enabled(io).ok_or(EventsError::Io)? {
        return Ok(false);
    }
    // A zero SMI_CMD means a platform with no legacy mode (5.2.9), which
    // should never read SCI_EN as clear.
    let port = u16::try_from(f.smi_cmd).map_err(|_| EventsError::NoModeSwitch)?;
    if port == 0 || f.acpi_enable == 0 {
        return Err(EventsError::NoModeSwitch);
    }
    if !io.write(port, 1, u16::from(f.acpi_enable)) {
        return Err(EventsError::Io);
    }
    let mut waited = 0;
    loop {
        if cnt.sci_enabled(io).ok_or(EventsError::Io)? {
            return Ok(true);
        }
        if waited >= wait_ms {
            return Err(EventsError::ModeTimeout);
        }
        io.sleep(POLL_MS);
        waited += POLL_MS;
    }
}
