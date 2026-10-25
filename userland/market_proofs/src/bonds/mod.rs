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

#[path = "../../../capsule_market/src/bonds/cache.rs"]
mod cache;
#[path = "../../../capsule_market/src/bonds/late.rs"]
mod late;
#[path = "../../../capsule_market/src/bonds/refresh.rs"]
mod refresh;
#[path = "../../../capsule_market/src/bonds/rows.rs"]
mod rows;
#[cfg(test)]
mod tests;

pub use cache::{BondCache, BondKey, Row, CACHE_MAX, QUEUE_MAX, REFRESH_AFTER};
pub use refresh::{Ask, STEP_MS};
