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

use super::domain_id_limit::domain_id_limit;
use super::state::{FIRST_DYNAMIC_DOMAIN_ID, STATE};

/// The lowest free id the units accept, or the limit itself when none is,
/// which `create_domain` refuses. A counter that never went back ran out
/// after 256 claims in one boot however many domains were live, and a driver
/// restarted that often would then find every claim refused.
pub fn allocate_domain_id() -> u64 {
    let limit = domain_id_limit();
    let state = STATE.lock();
    let first = FIRST_DYNAMIC_DOMAIN_ID as usize;
    (first..limit).find(|&i| !state.domains[i].used).unwrap_or(limit) as u64
}
