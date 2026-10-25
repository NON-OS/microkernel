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

//! Numbers retired under the stable-ABI rule: each keeps its value, so it
//! never comes to mean anything else, and answers EINVAL to every caller
//! before any dispatch. MBGL (MkBootLogo) gave the boot splash the firmware's
//! logo; the splash no longer shows one, and nothing else asked for it. Pure;
//! held in kernel_proofs.

use super::errnos::ERRNO_INVAL;

/// The retired numbers' tags, as abi/syscalls.toml still publishes them.
pub(crate) const RETIRED: [[u8; 4]; 1] = [*b"MBGL"];

/// EINVAL for a retired number; None for every other.
pub fn handle(nr: u64) -> Option<i64> {
    RETIRED.iter().any(|t| u64::from(u32::from_le_bytes(*t)) == nr).then_some(ERRNO_INVAL)
}
