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

//! The console lines that let a photo of `log` show where the card got to:
//! one when bring-up finishes, with CTRL and STATUS as the part left them,
//! and one each time the link changes, with the speed and duplex the PHY
//! negotiated. Autonegotiation takes seconds, so the link is reported as the
//! network stack polls it rather than at bring-up.

mod line;
mod link;
mod up;

pub use link::link;
pub use up::up;
