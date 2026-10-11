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

//! Whether the in-kernel virtio-rng may say it is available. It hands the
//! device kernel virtual addresses where DMA needs physical ones (the queue
//! and buffer statics in queue/core.rs, the legacy PFN in device/init.rs
//! taken from them), so the device never writes a byte where the driver
//! reads. Worse, the device would write to whatever physical memory those
//! numbers name. So the driver neither programs the device nor reports
//! itself available, and keygen's HARDWARE_ENTROPY_VERIFIED never rests on
//! it, until P9 moves it out of the kernel.

/// The driver can deliver bytes. False until P9.
pub(crate) const DELIVERS: bool = false;

/// Whether init may touch the device at all: bus mastering, queue setup.
pub(crate) fn may_program(delivers: bool) -> bool {
    delivers
}

/// Available only when the driver delivers and the device was set up.
pub(crate) fn available(delivers: bool, set_up: bool) -> bool {
    delivers && set_up
}
