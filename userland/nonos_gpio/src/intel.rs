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

//! Intel communities as a `Controller`. The pin number firmware gives is
//! not a pad index on these parts: where a pin lies (which community
//! window, which pad, which group and bit) comes from the caller's
//! `place`, built from a published layout or from what `_CRS` and `_DSD`
//! say; a pin it cannot place is refused. Arming leaves the pad's mode and
//! routing as firmware set them, and refuses a pad that is a native
//! function or has its input disabled.

use crate::controller::GpioError;
use crate::intel_bits::*;
pub use crate::intel_layout::{Community, IntelGpio, IntelPad};
use crate::trigger::{Polarity, Trigger};
use crate::window::Window;

impl<W: Window> IntelGpio<W> {
    /// The pad's community, the offset of its DW0, and where it sits.
    pub(crate) fn dw0(
        &mut self,
        pin: u16,
    ) -> Result<(&mut Community<W>, u32, IntelPad), GpioError> {
        let p = (self.place)(pin).ok_or(GpioError::Unplaced)?;
        let c = self.communities.get_mut(p.community).ok_or(GpioError::Unplaced)?;
        let at = c.window.read(PADBAR) + u32::from(p.pad) * c.stride;
        Ok((c, at, p))
    }

    pub(crate) fn arm_pin(&mut self, pin: u16, t: Trigger) -> Result<(), GpioError> {
        let (c, at, p) = self.dw0(pin)?;
        let v = c.window.read(at);
        if v & PMODE_MASK != 0 || v & RX_DISABLE != 0 {
            return Err(GpioError::Trigger);
        }
        let (ev, invert) = match (t.edge, t.polarity) {
            (true, Polarity::Both) => (3, false),
            (edge, polarity) => (u32::from(edge), polarity == Polarity::Low),
        };
        let v = (v & !(RX_EVCFG_MASK | RX_INVERT)) | ev << RX_EVCFG_SHIFT;
        c.window.write(at, if invert { v | RX_INVERT } else { v });
        let ie = c.gpi_ie + 4 * u32::from(p.group);
        let enabled = c.window.read(ie) | 1 << p.bit;
        c.window.write(ie, enabled);
        Ok(())
    }

    pub(crate) fn mask_pin(&mut self, pin: u16, masked: bool) -> Result<(), GpioError> {
        let (c, _, p) = self.dw0(pin)?;
        let ie = c.gpi_ie + 4 * u32::from(p.group);
        let v = c.window.read(ie);
        c.window.write(ie, if masked { v & !(1 << p.bit) } else { v | 1 << p.bit });
        Ok(())
    }
}
