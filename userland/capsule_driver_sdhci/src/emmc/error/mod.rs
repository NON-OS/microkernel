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

//! Why the eMMC host or card did not do what was asked.

mod kind;
mod reason;
mod wire_codes;
mod wire_status;

pub use kind::{EmmcError, EmmcResult};
pub use reason::reason;
pub use wire_codes::{CARD_ERROR_BASE, HOST_ERROR_BASE};
pub use wire_status::wire_status;
