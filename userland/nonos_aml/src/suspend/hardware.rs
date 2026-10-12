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

//! What entering S3 asks of the hardware, which the kernel owns: the FACS
//! waking vector, PM1's wake status and control, the GPE enables, and
//! whether the Global Lock is held. The capsule answers NotWired for each
//! until the kernel path exists; the proofs answer with a model platform.

use crate::sleep::SleepTypes;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwError {
    /// No kernel path for this yet.
    NotWired,
    /// The kernel refused it.
    Refused,
}

pub trait SleepHardware {
    /// Whether OSPM holds the Global Lock: firmware takes it on its way
    /// down and back, so the OS must not hold it across a sleep (ACPI 6.5,
    /// 5.2.10.1).
    fn global_lock_held(&self) -> bool;
    /// Where firmware jumps on resume: the FACS's Firmware_Waking_Vector,
    /// or X_Firmware_Waking_Vector where it is used (5.2.10).
    fn set_waking_vector(&mut self, address: u64) -> Result<(), HwError>;
    /// Clears WAK_STS in PM1 status, so the wake that follows is seen
    /// (4.8.3.1.1).
    fn clear_wake_status(&mut self) -> Result<(), HwError>;
    fn arm_gpe(&mut self, gpe: u16, on: bool) -> Result<(), HwError>;
    /// SLP_TYPa/b and SLP_EN written to PM1a/b control (16.1.3.1); returns
    /// once the machine has woken.
    fn sleep(&mut self, types: SleepTypes) -> Result<(), HwError>;
}
