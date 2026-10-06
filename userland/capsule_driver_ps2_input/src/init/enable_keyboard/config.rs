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
use super::cmd::cmd;
use super::data::data;
use super::read_byte::read_byte;
use crate::constants::{
    CONFIG_IRQ1, CONFIG_KBD_DISABLE, CONFIG_XLATE, CTL_DISABLE_KBD, CTL_READ_CONFIG,
    CTL_WRITE_CONFIG,
};
use crate::init::flush_output;

/// The configuration byte the keyboard needs: its interrupt on, its clock
/// on, and scan-code translation on. The keymap decodes set 1; a keyboard
/// powers up in set 2 and the controller translates only when bit 6 is set,
/// which firmware usually leaves set but does not have to. Linux
/// i8042_controller_init sets translation the same way. The aux bits are
/// left as they were; the mouse bring-up owns them.
pub fn keyboard_config(cfg: u8) -> u8 {
    (cfg | CONFIG_IRQ1 | CONFIG_XLATE) & !CONFIG_KBD_DISABLE
}

/// Read, fix and write back the configuration byte. The keyboard port is
/// disabled first so a keystroke cannot land in the output buffer between
/// the read command and its reply and be taken for the configuration.
/// A controller that does not answer the read is left as it is.
pub(super) fn configure(grant_id: u64) -> Result<(), &'static str> {
    cmd(grant_id, CTL_DISABLE_KBD)?;
    flush_output(grant_id);
    cmd(grant_id, CTL_READ_CONFIG)?;
    let Some(cfg) = read_byte(grant_id)? else { return Ok(()) };
    let want = keyboard_config(cfg);
    if want != cfg {
        cmd(grant_id, CTL_WRITE_CONFIG)?;
        data(grant_id, want)?;
    }
    Ok(())
}
