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

//! The client's saved network list in plaintext (the sealing around it needs
//! the TPM and the store), mounted as the client crate mounts it so its
//! `super::` references resolve.

#[path = "../../../nonos_wifi_client/src/saved/dropped.rs"]
mod dropped;
// The mounted source names items the store path (not mounted here) uses.
#[allow(dead_code)]
#[path = "../../../nonos_wifi_client/src/saved/error.rs"]
pub mod error;
#[allow(dead_code)]
#[path = "../../../nonos_wifi_client/src/saved/file.rs"]
mod file;
#[path = "../../../nonos_wifi_client/src/saved/list.rs"]
pub mod list;
#[path = "../../../nonos_wifi_client/src/saved/list_codec.rs"]
mod list_codec;
#[path = "../../../nonos_wifi_client/src/saved/list_put.rs"]
mod list_put;
#[path = "../../../nonos_wifi_client/src/saved/pass.rs"]
mod pass;
#[path = "../../../nonos_wifi_client/src/saved/pin.rs"]
mod pin;

#[cfg(test)]
mod evict_tests;
#[cfg(test)]
mod fault_tests;
#[cfg(test)]
mod pass_tests;
#[cfg(test)]
mod pin_tests;
#[cfg(test)]
mod saved_tests;
