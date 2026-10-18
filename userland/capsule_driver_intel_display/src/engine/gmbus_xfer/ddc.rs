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

//! DDC/CI over GMBUS, for an HDMI or DVI monitor: a write of the message to
//! the DDC/CI address, four bytes to GMBUS3 at a time (the first before the
//! command, the rest as the controller is ready for them), or a plain read
//! of the reply with no index cycle; each ended with a stop and the pins
//! let go. The write's sequence is this driver's reading of the GMBUS
//! registers, unverified against the PRM. Each transaction has one
//! deadline, XFER_US, that all its waits draw on.

use super::super::access::{Clock, Mmio};
use super::super::gmbus_regs::*;
use super::deadline::{Deadline, XFER_US};
use super::end::end;
use super::put::put;
use super::take::take;
use crate::ddc::bus::DdcBus;
use crate::ddc::caps_frame::CAPS_READ;
use crate::ddc::error::DdcError;
use crate::ddc::frame::DDC_CI_ADDR;

/// The most one DDC/CI message here holds: a Set VCP's seven bytes.
const MAX_SEND: usize = 8;
/// The most one reply here holds: a capabilities reply's 38 bytes, well
/// inside GMBUS1's byte count.
const MAX_TAKE: usize = CAPS_READ;

pub struct GmbusDdc<'a> {
    pub mmio: &'a mut dyn Mmio,
    pub clock: &'a dyn Clock,
    pub ddi: u32,
}

impl DdcBus for GmbusDdc<'_> {
    fn send(&mut self, bytes: &[u8]) -> Result<(), DdcError> {
        let p = pin(self.ddi).ok_or(DdcError::Bus)?;
        if bytes.is_empty() || bytes.len() > MAX_SEND {
            return Err(DdcError::Bus);
        }
        let (m, clock) = (&mut *self.mmio, self.clock);
        m.write(GMBUS0, p);
        let r = put(m, &Deadline::new(clock, XFER_US), DDC_CI_ADDR, bytes);
        end(m, clock, r.is_err());
        r.map_err(|_| DdcError::Bus)
    }

    fn receive(&mut self, out: &mut [u8]) -> Result<(), DdcError> {
        let p = pin(self.ddi).ok_or(DdcError::Bus)?;
        if out.is_empty() || out.len() > MAX_TAKE {
            return Err(DdcError::Bus);
        }
        let (m, clock) = (&mut *self.mmio, self.clock);
        m.write(GMBUS0, p);
        m.write(GMBUS1, plain_read_cmd(DDC_CI_ADDR, out.len()));
        let r = take(m, &Deadline::new(clock, XFER_US), out);
        end(m, clock, r.is_err());
        r.map_err(|_| DdcError::Bus)
    }
}
