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

//! The clipboard's `server` module, file for file as
//! capsule_clipboard/src/server/mod.rs lists it, less `runner`: the loop that
//! receives a frame, hands it to `route` and sends back what route wrote.

// The capsule's own lint choice, allowed on the include rather than restyled.
#[allow(clippy::manual_range_contains)]
#[path = "../../capsule_clipboard/src/server/handlers/mod.rs"]
pub mod handlers;
#[path = "../../capsule_clipboard/src/server/respond.rs"]
pub mod respond;
