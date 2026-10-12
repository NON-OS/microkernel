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
//! Proofs for the shared USB hub class, run on the host against the
//! library itself (a path dependency, so the code is the drivers' own).

#[cfg(test)]
mod fake;
#[cfg(test)]
mod hub_descriptor_tests;
#[cfg(test)]
mod hub_port_tests;
#[cfg(test)]
mod hub_route_tests;
#[cfg(test)]
mod parker_tests;
#[cfg(test)]
mod virt_claim_tests;
#[cfg(test)]
mod virt_list_tests;
#[cfg(test)]
mod virt_park_tests;
#[cfg(test)]
mod walk_more_tests;
#[cfg(test)]
mod walk_tests;
