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

use nonos_virtio::Mmio;

use crate::regs::Regs;

#[derive(Clone, Copy)]
pub enum Transport {
    /// The legacy register window, in an I/O or memory BAR.
    Legacy(Regs),
    /// The virtio 1.0 structures, each mapped on its own.
    Modern(Modern),
}

#[derive(Clone, Copy)]
pub struct Modern {
    /// struct virtio_net_config.
    pub device: Mmio,
    pub notify: Mmio,
    /// Each queue's doorbell offset in `notify`, by queue index (receive,
    /// transmit).
    pub doorbells: [usize; 2],
}
