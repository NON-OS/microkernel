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

//! Linux retries a failed MDIC transaction twice, 10 ms apart, on pch_mtp
//! and pch_ptp only (e1000_init_phy_params_pchlan sets retry_count 2 there).

use crate::constants::timeouts::MDIC_RETRY_MS;
use crate::constants::Family;
use crate::wait::sleep_ms;

pub fn with_retry(
    family: Family,
    mut f: impl FnMut() -> Result<u16, &'static str>,
) -> Result<u16, &'static str> {
    let tries = if family >= Family::PchMtp { 3 } else { 1 };
    let mut last = Err("MDIC not tried");
    for i in 0..tries {
        if i > 0 {
            sleep_ms(MDIC_RETRY_MS);
        }
        last = f();
        if last.is_ok() {
            break;
        }
    }
    last
}
