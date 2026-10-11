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

//! The requests that only take an object away (forget.rs).

use super::object::Object;
use super::ops::req;

/// Requests that only remove something.
pub fn is_destructor(object: Option<Object>, opcode: u16) -> bool {
    matches!(
        (object, opcode),
        (Some(Object::Buffer), req::BUFFER_DESTROY)
            | (Some(Object::ShmPool), req::POOL_DESTROY)
            | (Some(Object::Surface), req::SURFACE_DESTROY)
            | (Some(Object::Region), req::REGION_DESTROY)
            | (Some(Object::Subcompositor), req::SUBCOMPOSITOR_DESTROY)
            | (Some(Object::Subsurface), req::SUBSURFACE_DESTROY)
    )
}
