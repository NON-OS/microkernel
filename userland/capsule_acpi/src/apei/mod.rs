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

//! Hardware errors the platform reports (ACPI 6.5, chapter 18): the last
//! boot's fatal one from BERT, and every GHES source's records, polled or
//! on the Error Device's Notify, each logged and counted by the kernel.

mod phys;
mod read;
mod say;
mod serve;
mod start;
mod state;

pub use serve::{notified, tick};
pub use start::start;
pub use state::Apei;
