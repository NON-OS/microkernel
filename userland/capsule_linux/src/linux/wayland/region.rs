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

//! wl_region: the parts of a surface a client calls opaque, or that take
//! input. A guest's window is shown whole and takes input everywhere, so a
//! region changes nothing here. It is still an object: a client names it
//! in later requests, destroys it, and reuses its id, and an id never made
//! would leave every one of those unserved.

use crate::linux::guest::Guest;

use super::args::Args;
use super::object::Object;

pub fn create(guest: &mut Guest, args: &mut Args<'_>) {
    if let Some(id) = args.u32() {
        let _ = guest.objects.put(id, Object::Region);
    }
}
