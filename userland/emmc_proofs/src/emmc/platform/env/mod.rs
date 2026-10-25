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

//! `Mmio`, `Clock` and `Log` for the capsule's open path: its serial log and
//! uptime clock over the stand-in nonos_libc, and a register window that
//! reaches the model host plugged in on this thread.

mod mmio_window;
#[path = "../../../../../capsule_driver_sdhci/src/emmc/platform/env/serial_log.rs"]
mod serial_log;
#[path = "../../../../../capsule_driver_sdhci/src/emmc/platform/env/uptime_clock.rs"]
mod uptime_clock;

pub use mmio_window::MmioWindow;
pub use serial_log::SerialLog;
pub use uptime_clock::UptimeClock;
