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

//! How PSCI reads a power_state here, asked once at boot.

use crate::arch::aarch64::psci;

/// Whether CPU_SUSPEND takes power_state in the extended format, or why it
/// cannot be used at all.
pub(super) fn suspend_format() -> Result<bool, &'static [u8]> {
    let version = psci::psci_version();
    if !psci::is_present() || !version.answered() || (version.major, version.minor) < (0, 2) {
        // PSCI 0.1 numbers its calls from the tree and has no version call;
        // CPU_SUSPEND is not reached that way here.
        Err(&b"no PSCI 0.2 or later to enter them through"[..])
    } else if version.supports_features() {
        // 0.2 has no PSCI_FEATURES and always the original format; from 1.0
        // CPU_SUSPEND may be absent, and its flags say which format.
        match psci::cpu_suspend_features() {
            Ok(flags) => Ok(flags & 0b10 != 0),
            Err(_) => Err(&b"PSCI does not offer CPU_SUSPEND"[..]),
        }
    } else {
        Ok(false)
    }
}
