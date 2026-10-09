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

//! The buffers the search field, the value editor and the passphrase type
//! into, grouped as they sit in the capsule's `state` module so the buffer's
//! `super::cache` resolves the same way, and the formatting buffer the
//! results heading writes a query into.

#[path = "../../../capsule_settings/src/settings/state/cache.rs"]
pub mod cache;
#[path = "../../../capsule_settings/src/settings/state/edit_buffer.rs"]
pub mod edit_buffer;
#[path = "../../../capsule_settings/src/settings/state/edit_paste.rs"]
pub mod edit_paste;
#[path = "../../../capsule_settings/src/settings/ui/valbuf.rs"]
pub mod valbuf;
