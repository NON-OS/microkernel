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

use super::distributor::GicDistributor;
use super::frames;

/// The GICv3 distributor, and through `frames` the calling CPU's own
/// redistributor for the SGIs and PPIs that live there.
pub struct Gic {
    distributor: GicDistributor,
}

impl Gic {
    pub fn new(dist_base: u64) -> Self {
        Self { distributor: GicDistributor::new(dist_base) }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        self.distributor.init()?;
        super::cpu::init_gic_cpu()
    }

    pub fn enable_irq(&self, irq: u32) {
        if irq >= 32 {
            self.distributor.enable_irq(irq);
        } else if let Some(frame) = frames::this_cpu() {
            frame.enable_irq(irq);
        }
    }

    pub fn disable_irq(&self, irq: u32) {
        if irq >= 32 {
            self.distributor.disable_irq(irq);
        } else if let Some(frame) = frames::this_cpu() {
            frame.disable_irq(irq);
        }
    }

    pub fn set_priority(&self, irq: u32, priority: u8) {
        if irq >= 32 {
            self.distributor.set_priority(irq, priority);
        } else if let Some(frame) = frames::this_cpu() {
            frame.set_priority(irq, priority);
        }
    }

    pub fn set_config(&self, irq: u32, edge: bool) {
        if irq >= 32 {
            self.distributor.set_config(irq, edge);
        } else if let Some(frame) = frames::this_cpu() {
            frame.set_config(irq, edge);
        }
    }
}
