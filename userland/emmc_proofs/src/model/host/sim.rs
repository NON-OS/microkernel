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

//! The model host's state, and a host fresh from power on.

use std::cell::Cell;
use std::rc::Rc;

use super::super::card::{Card, CardCfg};
use super::super::sd::SdCard;
use super::cfg::HostCfg;
use super::types::{CmdRec, Desc};

pub struct Sim {
    pub cfg: HostCfg,
    pub(super) regs: [u8; 256],
    pub(super) int: u32,
    pub card: Card,
    pub sd: Option<SdCard>,
    pub(super) now: Rc<Cell<u64>>,
    /// DMA memory: (bus address, host pointer, length).
    pub mem: Vec<(u64, usize, usize)>,
    pub(super) tc_at: Option<u64>,
    pub(super) srst_left: u32,
    pub(super) power_ignores: u32,
    pub cmds: Vec<CmdRec>,
    pub violations: Vec<String>,
    /// Voltage select of every SD Bus Power on that stuck, in order.
    pub power_ons: Vec<u8>,
    pub fail_next_data: u32,
    pub tables: Vec<Vec<Desc>>,
    /// A card that lies: the R1 to this command index reports this state.
    pub lie: Option<(u8, u32)>,
}

impl Sim {
    pub fn new(cfg: HostCfg, card: CardCfg, now: Rc<Cell<u64>>) -> Self {
        let mut s = Self {
            power_ignores: cfg.power_ignores,
            cfg,
            regs: [0; 256],
            int: 0,
            card: Card::new(card),
            sd: None,
            now,
            mem: Vec::new(),
            tc_at: None,
            srst_left: 0,
            cmds: Vec::new(),
            violations: Vec::new(),
            power_ons: Vec::new(),
            lie: None,
            fail_next_data: 0,
            tables: Vec::new(),
        };
        s.reset_all();
        // Firmware left the host running at some setting.
        (s.regs[0x28], s.regs[0x29]) = (0x3e, 0x0b);
        s
    }
}
