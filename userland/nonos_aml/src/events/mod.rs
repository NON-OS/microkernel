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

//! The SCI and its events (ACPI 6.5, 4.8 and 5.6): the fixed PM1 events,
//! the general-purpose event blocks and their control methods, and the
//! switch into ACPI mode. Register access goes through `EventIo`; methods
//! run through the caller's evaluator.

mod acpi_mode;
mod bytes;
mod dispatch;
mod error;
mod fadt;
mod fadt_fields;
mod gas;
mod gpe_block;
mod handlers;
mod io;
mod native;
mod pending;
mod pm1;
mod pm1_bits;
mod pm1_cnt;
mod policy;
mod quiet;
mod rearm;
mod report;
mod sci;
mod start;
mod state;
mod storm;
mod storm_backoff;
mod wake;
mod wake_arm;

pub use acpi_mode::enter_acpi_mode;
pub use dispatch::Dispatch;
pub use error::EventsError;
pub use fadt::{parse as parse_fadt, Fadt, FadtError};
pub use gas::{Block, BlockError, SYSTEM_IO};
pub use gpe_block::GpeBlock;
pub use handlers::{handlers, Handler, Trigger};
pub use io::EventIo;
pub use pm1::{Pm1, GBL, PWRBTN, RTC, SLPBTN, TMR, WAK};
pub use pm1_cnt::{Pm1Cnt, GBL_RLS, SCI_EN, SLP_EN};
pub use policy::Policy;
pub use report::SciReport;
pub use state::Events;
pub use storm::{Storms, RUNS, WINDOW_MS};
pub use storm_backoff::{CUTS, FIRST_MS, LAST_MS};
pub use wake::wake_gpes;
