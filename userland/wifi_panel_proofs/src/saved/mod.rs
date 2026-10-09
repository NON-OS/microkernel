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

#[path = "../../../nonos_wifi_client/src/saved/list.rs"]
pub mod list;
#[path = "../../../nonos_wifi_client/src/saved/list_codec.rs"]
mod list_codec;

#[cfg(test)]
mod saved_tests;
