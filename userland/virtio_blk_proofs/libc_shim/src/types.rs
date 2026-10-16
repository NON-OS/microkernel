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

//! The broker's record types, as the libc defines them.

#[path = "../../../libc/src/broker/types/bar.rs"]
pub mod bar;
#[path = "../../../libc/src/broker/types/bus.rs"]
pub mod bus;
#[path = "../../../libc/src/broker/types/device.rs"]
pub mod device;
#[path = "../../../libc/src/broker/types/dma.rs"]
pub mod dma;
#[path = "../../../libc/src/broker/types/irq.rs"]
pub mod irq;
#[path = "../../../libc/src/broker/types/mmio.rs"]
pub mod mmio;
#[path = "../../../libc/src/broker/types/pio.rs"]
pub mod pio;
