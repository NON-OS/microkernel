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

mod any_mapped;
mod map_fresh;
mod map_kernel;
mod map_loop;
mod release_va;
mod reserve_va;
mod rollback_mapped_pages;
mod unmap_range;

pub(super) use any_mapped::any_mapped;
pub(super) use map_fresh::map_fresh;
pub(super) use release_va::release_va;
pub(super) use reserve_va::reserve_va;
pub(super) use rollback_mapped_pages::rollback_mapped_pages;
pub(super) use unmap_range::unmap_range;
