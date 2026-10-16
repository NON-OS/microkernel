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

//! A monitor control read or set on a claimed output (`ddc_bus.rs` finds
//! its bus), and DDC/CI's refusals as the wire's errnos.

use nonos_gfx_wire::errno::{EINVAL, ENOSYS, ETIMEDOUT};
use nonos_gfx_wire::Errno;

use super::brightness_wire::{EIO, EPERM};
use super::frames::Frames;
use super::intel::Intel;
use super::vcp_wire::Controls;
use crate::ddc::error::DdcError;
use crate::ddc::pace::Pacer;
use crate::ddc::reply::Reading;
use crate::ddc::transact;
use crate::engine::access::{Clock, Mmio};

impl<M: Mmio, F: Frames, C: Clock> Controls for Intel<M, F, C> {
    fn vcp(&mut self, output: u16, code: u8) -> Result<(u16, u16), Errno> {
        self.vcp_read(output, code).map(|r| (r.present, r.max))
    }

    fn set_vcp(&mut self, output: u16, code: u8, value: u16) -> Result<(), Errno> {
        self.vcp_write(output, code, value)
    }
}

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    pub fn vcp_read(&mut self, output: u16, code: u8) -> Result<Reading, Errno> {
        let o = output as usize;
        self.on_bus(output, |b, c, p| transact::read(b, c, slot(p, o)?, code))
    }

    pub fn vcp_write(&mut self, output: u16, code: u8, value: u16) -> Result<(), Errno> {
        let o = output as usize;
        self.on_bus(output, |b, c, p| transact::write(b, c, slot(p, o)?, code, value))
    }
}

/// The output's pacer; `on_bus` found its head, so it has one.
fn slot(p: &mut [Pacer], o: usize) -> Result<&mut Pacer, DdcError> {
    p.get_mut(o).ok_or(DdcError::Bus)
}

pub(super) fn errno(e: DdcError) -> Errno {
    match e {
        DdcError::NotAllowed | DdcError::ReadOnly => EPERM,
        DdcError::Value => EINVAL,
        DdcError::Unsupported | DdcError::Absent => ENOSYS,
        DdcError::NoClock => ETIMEDOUT,
        DdcError::Bus | DdcError::Length | DdcError::Checksum | DdcError::Reply => EIO,
    }
}
