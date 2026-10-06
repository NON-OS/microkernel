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

//! `src/security/capsule_manifest`: its cursor, errors, schema and decoder.

#[path = "../../../src/security/capsule_manifest/cursor.rs"]
pub mod cursor;
#[path = "../../../src/security/capsule_manifest/decode/mod.rs"]
pub mod decode;
#[path = "../../../src/security/capsule_manifest/error.rs"]
pub mod error;
#[path = "../../../src/security/capsule_manifest/schema/mod.rs"]
pub mod schema;
