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

//! Host-runnable proofs for the RTL8153 driver.

extern crate alloc;

#[path = "../../capsule_driver_rtl8153/src/r8153/mod.rs"]
pub mod r8153;

#[cfg(test)]
mod bind_tests;
#[cfg(test)]
mod chip;
#[cfg(test)]
mod enable_tests;
#[cfg(test)]
mod fail_tests;
#[cfg(test)]
mod link_tests;
#[cfg(test)]
mod ocp_tests;
#[cfg(test)]
mod phy_window_tests;
#[cfg(test)]
mod queue_tests;
#[cfg(test)]
mod rtl8153b_tests;
#[cfg(test)]
mod rx_fuzz_tests;
#[cfg(test)]
mod rx_tests;
#[cfg(test)]
mod tx_tests;
#[cfg(test)]
mod version_tests;
#[cfg(test)]
mod wait_tests;
