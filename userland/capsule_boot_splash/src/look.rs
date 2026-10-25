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

//! What every repaint of the splash shows besides its status line: the
//! attestation status and its verdict, and the layout for the canvas.

use nonos_libc::AttestStatus;

use crate::attest_lines::Chain;
use crate::layout_fit::Layout;

pub(crate) struct Look {
    pub att: AttestStatus,
    /// What the boot recorded, for the panel; None when the kernel gave none.
    pub chain: Option<Chain>,
    /// Where the panel, the warning's row and the status line sit.
    pub fit: Layout,
    /// The kernel says device DMA is not confined by hardware (no IOMMU):
    /// the warning is drawn in its row.
    pub dma_unconfined: bool,
}

/// The panel's record from MkAttestStatus's `rc` and what it wrote in `att`.
pub(crate) fn chain(att: &AttestStatus, rc: i64) -> Option<Chain> {
    let c = Chain {
        secure_boot: att.secure_boot,
        kernel_sig_ok: att.kernel_sig_ok,
        zk_verified: att.zk_verified,
    };
    (rc == 0).then_some(c)
}
