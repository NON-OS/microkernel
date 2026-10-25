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

//! The keyring's wire types, and the ops it retired, from capsule source.

#[path = "../../../capsule_keyring/src/protocol/encode.rs"]
pub mod encode;
#[path = "../../../capsule_keyring/src/protocol/errno.rs"]
pub mod errno;
#[path = "../../../capsule_keyring/src/protocol/gated.rs"]
pub mod gated;
#[path = "../../../capsule_keyring/src/protocol/retired.rs"]
pub mod retired;
#[path = "../../../capsule_keyring/src/protocol/types.rs"]
pub mod types;
