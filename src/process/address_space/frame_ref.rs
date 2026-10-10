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

use crate::memory::addr::PhysAddr;
use crate::memory::page_info::{self, PageFlags};

pub(super) fn share_frame(pa: PhysAddr) -> Result<(), &'static str> {
    if page_info::get_page_info(pa).is_none() {
        page_info::add_page(pa, None, PageFlags::USER).map_err(|e| e.as_str())?;
    }
    page_info::increment_ref_count(pa).map(|_| ()).map_err(|e| e.as_str())
}
