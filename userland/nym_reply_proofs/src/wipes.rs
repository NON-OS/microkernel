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

//! net.nym's secrets and where they are wiped: the identity, the gateway key
//! in its slot, the handshake and HKDF, and what no getter may hand out.

#[path = "hkdf_tests.rs"]
pub(crate) mod hkdf_tests;
#[path = "hkdf_wipe_tests.rs"]
pub(crate) mod hkdf_wipe_tests;
#[path = "key_getter_tests.rs"]
pub(crate) mod key_getter_tests;
#[path = "key_slot_tests.rs"]
pub(crate) mod key_slot_tests;
#[path = "secret_wipe_tests.rs"]
pub(crate) mod secret_wipe_tests;
#[path = "wipe_glue_tests.rs"]
pub(crate) mod wipe_glue_tests;
