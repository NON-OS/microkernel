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

//! Reading a PSCI power_state without changing it (DEN0022, CPU_SUSPEND,
//! recalled, unverified): StateType is bit 16 in the original format and
//! bit 30 in the extended one; zero is standby or retention, one powerdown.

/// True when the state keeps this CPU's context, so CPU_SUSPEND returns to
/// its caller on a wake. A powerdown state instead resumes at an entry
/// point with the MMU off, which the kernel does not offer yet.
pub fn is_standby(param: u32, extended: bool) -> bool {
    let state_type = if extended { 1 << 30 } else { 1 << 16 };
    param & state_type == 0
}
