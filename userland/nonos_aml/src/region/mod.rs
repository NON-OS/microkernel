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

//! The pure arithmetic of fields and regions (section 5.5.2.4 and 19.6.48):
//! which accesses a field takes, how a partial write is merged, and the
//! Global Lock's protocol (section 5.2.10.1). No I/O is done here.

mod bits;
mod global_lock;
mod plan;
mod space;
mod update;
mod width;

pub use bits::{get_bits, set_bits};
pub use global_lock::{acquire_step, release_step};
pub use plan::{plan, Unit};
pub use space::Space;
pub use update::{merge, UpdateRule};
pub use width::access_width;
