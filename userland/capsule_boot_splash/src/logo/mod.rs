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

//! The firmware's boot logo (MkBootLogo), drawn where the firmware left it so
//! the screen goes from the firmware's logo to the splash without a flash.
//! Everything here but `fetch` is plain computation, proven on the host.

pub mod bmp;
pub mod fetch;
pub mod place;
pub mod prepare;
pub mod record;
pub mod resample;
pub mod shown;
pub mod turn;

pub use prepare::prepare;
pub use shown::Shown;
