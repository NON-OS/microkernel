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

use nonos_libc::mk_idle_ms;

/// How long a device with no input register to poll is given after RESET.
pub(super) const SETTLE_MS: u64 = 20;
/// The pause after SET_POWER(ON) before RESET, from Linux i2c_hid_set_power.
pub(super) const POWER_ON_MS: u64 = 60;
/// The pause before a NACKed power-on is sent again; Linux sleeps 400 to
/// 500 us, and a millisecond is the finest sleep there is here.
pub(super) const RETRY_MS: u64 = 1;

/// A fixed pause for a device with no addressable reset sentinel. A real
/// sleep: the 2048 yields this used to be took no time at all on an idle
/// machine, and a core while they lasted.
pub(super) fn settle() {
    let _ = mk_idle_ms(SETTLE_MS);
}
