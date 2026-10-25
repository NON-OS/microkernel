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

use super::super::error::RngResult;
use super::generate::{random_u32, random_u32_secure};

#[track_caller]
pub fn random_range(n: u32) -> u32 {
    if n <= 1 {
        return 0;
    }

    if n.is_power_of_two() {
        return random_u32() & (n - 1);
    }

    let threshold = u32::MAX - (u32::MAX % n);

    loop {
        let r = random_u32();
        if r < threshold {
            return r % n;
        }
    }
}

pub fn random_range_secure(n: u32) -> RngResult<u32> {
    if n <= 1 {
        return Ok(0);
    }

    if n.is_power_of_two() {
        return Ok(random_u32_secure()? & (n - 1));
    }

    let threshold = u32::MAX - (u32::MAX % n);

    loop {
        let r = random_u32_secure()?;
        if r < threshold {
            return Ok(r % n);
        }
    }
}
