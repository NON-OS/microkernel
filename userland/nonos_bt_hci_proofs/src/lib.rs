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

//! Proofs for the HCI host and its USB transport. Every module is
//! test-only.

#[cfg(test)]
mod acl_tests;
#[cfg(test)]
mod address_tests;
#[cfg(test)]
mod bringup_tests;
#[cfg(test)]
mod connect_cancel_tests;
#[cfg(test)]
mod connect_tests;
#[cfg(test)]
mod descriptor_tests;
#[cfg(test)]
mod encrypt_end_tests;
#[cfg(test)]
mod encrypt_tests;
#[cfg(test)]
mod filter_tests;
#[cfg(test)]
mod flow_tests;
#[cfg(test)]
mod joiner_tests;
#[cfg(test)]
mod scan_tests;
#[cfg(test)]
mod sim;
#[cfg(test)]
mod transport_tests;
#[cfg(test)]
mod uart;
#[cfg(test)]
mod update_tests;
