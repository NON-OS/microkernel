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

//! The bus DDC/CI is spoken on for a claimed output, and on nothing else:
//! the route its own transcoder drives, read back from the engine, so a
//! request can reach no bus but that output's. The eDP panel has no DDC/CI
//! here (its backlight is the PWM's); only Gen9's GMBUS and AUX registers
//! are cited.

use nonos_gfx_wire::errno::ENOSYS;
use nonos_gfx_wire::Errno;

use super::ddc_of::errno;
use super::frames::Frames;
use super::intel::Intel;
use crate::ddc::bus::DdcBus;
use crate::ddc::error::DdcError;
use crate::ddc::pace::Pacer;
use crate::dp::i2c_ddc::AuxDdc;
use crate::engine::access::{Clock, Mmio};
use crate::engine::aux_regs::Ddi;
use crate::engine::gen::GEN9;
use crate::engine::gmbus_xfer::ddc::GmbusDdc;
use crate::engine::pipe::Transcoder;
use crate::engine::port::Port;
use crate::engine::route::{self, Link};

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    pub(super) fn on_bus<T>(
        &mut self,
        output: u16,
        f: impl FnOnce(&mut dyn DdcBus, &dyn Clock, &mut [Pacer]) -> Result<T, DdcError>,
    ) -> Result<T, Errno> {
        let t = self.head(output)?.out.transcoder;
        if t == Transcoder::Edp || *self.gen != GEN9 {
            return Err(ENOSYS);
        }
        let Intel { m, clock, ddc, .. } = self;
        let r = route::read(m, t).ok_or(ENOSYS)?;
        let got = match r.link {
            Link::Dp { .. } => {
                let ddi =
                    [Ddi::A, Ddi::B, Ddi::C, Ddi::D].get(r.ddi as usize).copied().ok_or(ENOSYS)?;
                let mut p = Port { mmio: m, ddi, dpll: 0, enhanced: false, clock };
                f(&mut AuxDdc { aux: &mut p }, clock, ddc)
            }
            Link::Hdmi | Link::Dvi => f(&mut GmbusDdc { mmio: m, clock, ddi: r.ddi }, clock, ddc),
        };
        got.map_err(errno)
    }
}
