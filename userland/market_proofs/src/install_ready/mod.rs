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

//! The gate's files, mounted where the capsule keeps them.

#[path = "../../../capsule_market/src/install_ready/arch.rs"]
pub mod arch;

#[path = "../../../capsule_market/src/install_ready/bond.rs"]
pub mod bond;

#[path = "../../../capsule_market/src/install_ready/checks.rs"]
pub mod checks;

#[path = "../../../capsule_market/src/install_ready/market.rs"]
pub mod market;

#[path = "../../../capsule_market/src/install_ready/pin.rs"]
pub mod pin;

/* It reads the chain hosts, which only the tests mount. */
#[cfg(test)]
#[path = "../../../capsule_market/src/install_ready/ready.rs"]
pub mod ready;
#[cfg(test)]
pub use ready::ready;
