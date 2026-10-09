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

use super::config::configure;
use super::enable_port::enable_port;
use super::reset::reset;
use crate::init::enable_scanning;

/// Configure the controller for the keyboard, enable its port and start
/// scanning. Linux on x86 does not reset the keyboard by default, and some
/// EC-emulated keyboards misbehave when reset, so the reset is the recovery
/// for a keyboard that does not acknowledge scanning, not the first step. A
/// keyboard that answers neither is tolerated: the controller is up and a
/// keyboard plugged in later works.
pub fn enable_keyboard(grant_id: u64) -> Result<(), &'static str> {
    configure(grant_id)?;
    enable_port(grant_id)?;
    if enable_scanning(grant_id)? {
        return Ok(());
    }
    if reset(grant_id)? {
        let _ = enable_scanning(grant_id)?;
    }
    Ok(())
}
