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

//! The caps a vendor capsule_linux supervisor runs with. Since C5a
//! the run role holds no LocalSign, so a supervisor still holding it was
//! built before C5a and seals nothing a verifier accepts.

/// LINUX_CAPS, the run role's caps.
pub const LINUX_CAPS: u64 = 0x1_0400_1979;
/// The run.net role's caps: LINUX_CAPS with Network.
pub const LINUX_NET_CAPS: u64 = 0x1_0400_197D;
/// The policy mask bit that grants the run.net role.
pub const NET_ANON: u64 = 1 << 1;

/// The one caps value a record sealed under a policy with `mask` may carry.
pub const fn supervisor_caps(mask: u64) -> u64 {
    if mask & NET_ANON != 0 {
        LINUX_NET_CAPS
    } else {
        LINUX_CAPS
    }
}
