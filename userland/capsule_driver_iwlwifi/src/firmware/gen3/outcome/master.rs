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

//! The Bus Master step. A capsule that died without its Drop (a panic, a
//! kill, an allocation failure) can leave the device's DMA engines armed at
//! memory the kernel freed at release. The release cleared Bus Master, so
//! nothing moved; turning it on again before the engines stop would let them
//! write there. So Bus Master goes on only once the device's own bus master
//! was told to stop and read back stopped and the chip was reset
//! (`stop_device`: STOP_MASTER, MASTER_DISABLED, SW_RESET), and only right
//! before the firmware is booted: after the part was judged runnable and its
//! regions mapped, and again before each restart.

use super::super::region::Clock;
use super::super::start::stop_device;
use super::kinds::Failure;
use crate::regs::Mmio;

/// Why Bus Master did not go on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MasterError {
    /// The device's bus master never read back stopped.
    NotStopped,
    /// The broker refused the PCI command write.
    Refused,
}

/// Stop the device and see it stopped, then `enable` Bus Master.
pub fn master_on<M: Mmio, C: Clock>(
    m: &M,
    c: &mut C,
    enable: impl FnOnce() -> bool,
) -> Result<(), Failure> {
    if !stop_device(m, c) {
        return Err(Failure::Master(MasterError::NotStopped));
    }
    if !enable() {
        return Err(Failure::Master(MasterError::Refused));
    }
    Ok(())
}
