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

// The reply encoders under test resolve `super::util` and `super::path`, so
// including them at the crate root leaves those paths dangling. Declaring the
// two support modules as siblings here gives the real handler sources the
// module shape they compile against inside the capsule, which lets the wire
// proofs call the production encoder instead of a copy of it.

#[path = "../../capsule_vfs/src/server/handlers/path/mod.rs"]
pub mod path;
#[path = "../../capsule_vfs/src/server/handlers/util.rs"]
pub mod util;

#[path = "../../capsule_vfs/src/server/handlers/journal.rs"]
pub mod journal;
#[path = "../../capsule_vfs/src/server/handlers/search.rs"]
pub mod search;
