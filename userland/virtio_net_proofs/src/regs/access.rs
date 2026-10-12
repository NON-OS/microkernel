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

//! The driver's `Regs`, answered from the model register file.

use super::model::with;
use crate::constants::{LEG_MAC, LEG_QUEUE_NOTIFY, LEG_QUEUE_NUM, LEG_QUEUE_PFN, LEG_QUEUE_SEL};
use crate::constants::{LEG_STATUS, NET_CFG_STATUS};

const LINK_AT: usize = LEG_MAC + NET_CFG_STATUS;

#[derive(Clone, Copy)]
pub struct Regs;

impl Regs {
    pub fn r8(self, off: usize) -> u8 {
        with(|m| match off {
            LEG_STATUS if m.reset_lag > 0 => {
                m.reset_lag -= 1;
                0x0F
            }
            LEG_STATUS => m.status,
            _ => 0xFF,
        })
    }

    pub fn r16(self, off: usize) -> u16 {
        with(|m| match off {
            LEG_QUEUE_NUM => m.qmax.get(usize::from(m.sel)).copied().unwrap_or(0),
            LINK_AT => m.link,
            _ => 0xFFFF,
        })
    }

    pub fn w8(self, off: usize, value: u8) {
        with(|m| m.status = if off == LEG_STATUS { value } else { m.status });
    }

    pub fn w16(self, off: usize, value: u16) {
        with(|m| match off {
            LEG_QUEUE_SEL => m.sel = value,
            LEG_QUEUE_NOTIFY => m.rung.push(value),
            _ => {}
        });
    }

    pub fn w32(self, off: usize, value: u32) {
        let sel = with(|m| usize::from(m.sel));
        with(|m| m.pfn.get_mut(sel).filter(|_| off == LEG_QUEUE_PFN).map(|p| *p = Some(value)));
    }
}
