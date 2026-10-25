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

//! The shell's power-off frame (G-53), on `acpi`. The name `power.off` is
//! capsule_power's; this capsule answers the same frame on its own inbox,
//! which HELD opens to more than the shell, so the sender is checked here. A request that
//! passes runs at once, \_TTS(5) and \_PTS(5) then AdminShutdown, inside
//! the shell's 5 s wait as long as firmware's methods are quick (F-111);
//! only a refusal is answered.

use super::{power_off, serve, Why};
use crate::boot::Acpi;
use super::wire::{refusal, well_formed};
use crate::server::caller::runs;
use alloc::vec::Vec;

const SHELL: &[u8] = b"desktop_shell";
const EPERM: i32 = -1;
const EINVAL: i32 = -22;

pub fn requested(acpi: &mut Acpi, req: &[u8], sender: u32) -> Vec<u8> {
    let status = if !well_formed(req) {
        EINVAL
    } else if sender == 0 || !runs(sender, SHELL) {
        EPERM
    } else {
        power_off(Why::Shell);
        serve(acpi).unwrap_or(0) as i32
    };
    refusal(req, status).to_vec()
}
