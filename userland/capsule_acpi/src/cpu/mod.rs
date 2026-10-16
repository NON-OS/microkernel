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

//! The processors' performance and idle tables (ACPI 6.5, 8.4), read for
//! the kernel. What runs at which frequency, and how deep a core sleeps,
//! is the kernel's to decide.

mod caps;
mod caps_kernel;
mod find;
mod lpi_record;
mod lpi_say;
mod notify;
mod osc_report;
mod record;
mod record_reg;
mod report;
mod say;
mod say_t;
mod say_ids;
mod send;
mod state;

pub use find::find;
pub use notify::notified;
pub use state::Cpus;
