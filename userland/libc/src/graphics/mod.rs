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

// Only the display-dimensions query is still served. Surface create, map,
// present, destroy, the cursor and the display list were retired in favour of
// the surface registry, and their wrappers here called syscall numbers the
// kernel no longer defines, so they could not succeed.
mod boot_logo;
mod display_dimensions;

pub use boot_logo::{mk_boot_logo, BOOT_LOGO_FILE, BOOT_LOGO_RECORD, BOOT_LOGO_RECORD_LEN};
pub use display_dimensions::{
    nonos_display_dimensions, nonos_display_physical_mm, nonos_display_refusal,
};
