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
//! What real controllers do that QEMU's does not: an extended capability
//! list far into BAR0, separate USB 2 and USB 3 port ranges, USB 3 ports
//! that enable themselves, PORTSC bits that act when written back, a page
//! size register, 32-bit-only DMA, and 64-byte contexts.
mod caps;
mod caps_tests;
mod context_tests;
mod dma_tests;
mod interval_tests;
mod legacy_tests;
mod page_tests;
mod port_number_tests;
mod portsc_tests;
mod regions;
mod speed_tests;
mod usb3_reset_tests;
mod warm_reset_tests;
