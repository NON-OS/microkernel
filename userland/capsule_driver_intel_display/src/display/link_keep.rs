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

//! Link maintenance for a DisplayPort output with mode setting (PLAN G 12;
//! DisplayPort 1.4a link maintenance, as `dp::link_check` cites): NONOS
//! takes no interrupts, so STATE reads the lane status over AUX and, when
//! the lanes no longer hold, trains the link again at the mode shown, on
//! the surfaces shown. eDP keeps the boot DPLL and is left to its panel.

use super::frames::Frames;
use super::head::Head;
use super::mode_light::light;
use crate::dp::link_check::link_lost;
use crate::engine::access::{Clock, Mmio};
use crate::engine::aux_regs::Ddi;
use crate::engine::port::Port;
use crate::engine::route::{self, Link};

/// None while the link holds (or the output is not DisplayPort with mode
/// setting); Some(true) when it was lost and is back, Some(false) when it
/// was lost and could not be brought back.
pub fn keep_link<F: Frames>(m: &mut dyn Mmio, clock: &dyn Clock, h: &mut Head<F>) -> Option<bool> {
    let ms = h.modeset?;
    let Link::Dp { lanes } = route::read(m, ms.lit.t)?.link else { return None };
    let ddi = [Ddi::A, Ddi::B, Ddi::C, Ddi::D].get(ms.lit.ddi as usize).copied()?;
    let mut port = Port { mmio: m, ddi, dpll: ms.lit.dpll, enhanced: false, clock };
    if !link_lost(&mut port, lanes).ok()? {
        return None;
    }
    let size = (h.scan.width, h.scan.height);
    Some(light(m, clock, &mut h.scan, (&ms, &ms.now), size, h.bases).is_ok())
}
