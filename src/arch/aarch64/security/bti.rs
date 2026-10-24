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

mod built;
mod control;
mod guard;
mod landing;
mod pad;

pub use built::{built_with_bti, built_with_pac_ret};
pub use control::{init_bti, kernel_guarded};
pub use guard::BtiGuard;
pub use landing::check_bti_landing_pad;
pub use pad::is_bti_landing_pad;
