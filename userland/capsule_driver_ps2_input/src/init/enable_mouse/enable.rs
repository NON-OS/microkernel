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
use super::command::command;
use super::detect_wheel::detect_wheel;
use super::mouse_command::mouse_command;
use super::read_data::read_config;
use super::write_data::write_data;
use crate::constants::{
    CONFIG_AUX_DISABLE, CONFIG_IRQ12, CTL_DISABLE_KBD, CTL_ENABLE_AUX, CTL_READ_CONFIG,
    CTL_WRITE_CONFIG, MOUSE_ENABLE_REPORTING, MOUSE_SET_DEFAULTS,
};
use crate::init::{flush_output, keyboard_config};

/// Bring the mouse up and return whether it negotiated the IntelliMouse scroll
/// wheel (a 4-byte report). The wheel probe runs before reporting is enabled so
/// the sample-rate writes are not interleaved with movement packets.
pub fn enable_mouse(grant_id: u64) -> Result<bool, &'static str> {
    // The keyboard is scanning by now, and its bytes carry the same status
    // as the configuration byte, so its port is held off until the write
    // below turns it back on (Linux i8042_controller_init holds both).
    command(grant_id, CTL_DISABLE_KBD)?;
    command(grant_id, CTL_ENABLE_AUX)?;
    flush_output(grant_id);
    command(grant_id, CTL_READ_CONFIG)?;
    let cfg = read_config(grant_id)?;
    command(grant_id, CTL_WRITE_CONFIG)?;
    // The keyboard's own bits stay as its bring-up set them (interrupt,
    // clock, translation); the aux clock and interrupt are turned on.
    write_data(grant_id, (keyboard_config(cfg) | CONFIG_IRQ12) & !CONFIG_AUX_DISABLE)?;
    mouse_command(grant_id, MOUSE_SET_DEFAULTS)?;
    let wheel = detect_wheel(grant_id).unwrap_or(false);
    mouse_command(grant_id, MOUSE_ENABLE_REPORTING)?;
    Ok(wheel)
}
