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

//! Suspend, the firmware's half: S3 (ACPI 6.5, 7.4 and 16.1) with its wake
//! devices, and suspend-to-idle through the Low Power S0 Idle device's
//! _DSM. Pure: the PM1, FACS and GPE writes go through `SleepHardware`,
//! and nothing here is reached until the capsule's switch is on.

mod hardware;
mod idle;
mod lps0;
mod outcome;
mod prw;
mod resume;
mod s3;
mod unwake;
mod vendor;
mod wake;

pub use hardware::{HwError, SleepHardware};
pub use idle::{
    idle_enter, idle_exit, Call, DISPLAY_OFF, DISPLAY_ON, ENTRY, EXIT, MS_ENTRY, MS_EXIT,
};
pub use lps0::{find_lps0, Lps0};
pub use outcome::{Outcome, SuspendError};
pub use prw::{read_prw, Prw};
pub use s3::{suspend_s3, S3};
pub use vendor::{Vendor, INTEL_UUID, MICROSOFT_UUID};
pub use wake::{Armed, Skip};
