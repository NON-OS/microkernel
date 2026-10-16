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

//! The default receive queue of an AX210-family part (Linux v6.12 pcie/rx.c
//! for `IWL_DEVICE_FAMILY_AX210`).
//!
//! The host posts buffers on the free ring (`iwl_rx_transfer_desc`: buffer id,
//! address); the device fills one, writes a completion descriptor
//! (`iwl_rx_completion_desc`: buffer id, fragmented flag) on the used ring and
//! advances the closed index in the status word. The host walks the used ring
//! up to that index, takes each buffer and posts it again. The free ring's
//! write index goes to the device in multiples of eight, and only after the
//! ALIVE interrupt: the firmware configures the receive engine itself and
//! Linux restocks then (`iwl_pcie_rxmq_restock` on ALIVE).
//!
//! Everything the device writes is checked: a buffer id that is zero, out of
//! range or not currently posted is refused (Linux forces an NMI there), and
//! the closed index is masked to the ring.

mod queue;
mod take;

pub use queue::{RxQueue, Taken};
