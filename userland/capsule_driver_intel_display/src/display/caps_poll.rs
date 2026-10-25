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

//! Each claimed output's capabilities string read a step a poll while its
//! monitor is in (`ddc/caps_fetch.rs`), on the output's own bus
//! (`ddc_bus.rs`), until it is read or refused; the eDP panel has no DDC/CI
//! here and is passed by.

use super::frames::Frames;
use super::intel::{Intel, MAX_HEADS};
use super::plug::Plug;
use crate::ddc::error::DdcError;
use crate::engine::access::{Clock, Mmio};

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    /// One step on each output still reading; the hot-plug tick calls it.
    pub fn poll_caps(&mut self) {
        for o in 0..MAX_HEADS {
            let here = self.heads[o].as_ref().is_some_and(|h| h.plug != Plug::Out);
            if !here || self.ddc[o].caps.settled() {
                continue;
            }
            let _ = self.on_bus(o as u16, |b, c, p| {
                let now = c.now_us().ok_or(DdcError::NoClock)?;
                p.get_mut(o).ok_or(DdcError::Bus)?.fetch_caps(b, now);
                Ok(())
            });
        }
    }
}
