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

use super::call::call;
use super::ids::{ARCH_FEATURES, V1_1, VERSION};
use crate::arch::aarch64::psci::{self, conduit};

/// The firmware's SMCCC version, or `None` for 1.0 or no firmware.
///
/// SMCCC_VERSION may only be called once PSCI_FEATURES has said it exists:
/// a PSCI 0.2 firmware, which has no PSCI_FEATURES, speaks SMCCC 1.0 and
/// does not implement it (DEN0028, discovery).
pub fn version() -> Option<u32> {
    let via = conduit()?;
    psci::features(VERSION).ok()?;
    let answer = call(via, VERSION, [0; 3]);
    u32::try_from(answer).ok()
}

/// SMCCC_ARCH_FEATURES for `func`, raw, or `None` where the firmware is
/// older than 1.1 and so has no such call to ask.
pub fn arch_features(func: u32) -> Option<i64> {
    let via = conduit()?;
    if version()? < V1_1 {
        return None;
    }
    Some(call(via, ARCH_FEATURES, [u64::from(func), 0, 0]))
}
