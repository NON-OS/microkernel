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

//! The transmitter half of a DDI, as the trainer drives it: the pattern on
//! DP_TP_CTL, the drive as DDI_BUF_CTL's translation entry.

use super::access::{rmw, Expiry, Until};
use super::ddi::{buf_ctl, entry, tp_ctl, NORMAL, PATTERN_MASK, TPS1, TPS2, TPS3};
use super::ddi::{TRANS_MASK, TRANS_SHIFT};
use super::port::Port;
use crate::dp::lane::Drive;
use crate::dp::train::{Link, Pattern, TrainError};

impl Link for Port<'_> {
    fn start(&mut self, mbps: u32, lanes: u32) -> Result<(), TrainError> {
        self.bring_up(mbps, lanes).map_err(|_| TrainError::Transmitter)
    }

    fn pattern(&mut self, p: Pattern) {
        let bits = match p {
            Pattern::Tps1 => TPS1,
            Pattern::Tps2 => TPS2,
            Pattern::Tps3 => TPS3,
            Pattern::Off => NORMAL,
        };
        let r = tp_ctl(self.index());
        rmw(self.mmio, r, PATTERN_MASK, bits);
    }

    fn drive(&mut self, d: Drive) {
        let e = entry(d) << TRANS_SHIFT;
        let r = buf_ctl(self.index());
        rmw(self.mmio, r, TRANS_MASK, e);
    }

    fn wait_us(&mut self, us: u32) {
        let until = Until::new(self.clock, us as u64);
        while !until.expired() {}
    }

    fn stop(&mut self) {
        let _ = self.take_down();
    }

    fn now_us(&self) -> Option<u64> {
        self.clock.now_us()
    }
}
