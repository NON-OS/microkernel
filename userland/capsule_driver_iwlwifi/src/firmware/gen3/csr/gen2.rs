// NONOS Operating System (AGPL-3.0-or-later)

//! What else a family's layout decides, where the 22000 family (`Gen2`)
//! differs from the AX210 and BZ families, as Linux v6.12 has it:
//!
//! - UMAC peripheral registers sit at `umac_prph_offset`, 0x300000 from
//!   AX210 on and none before (cfg/22000.c sets none), and the HBUS window
//!   decodes 24 address bits from AX210 on, 20 before
//!   (`iwl_trans_pcie_prph_msk`);
//! - the command queue is `IWL_CMD_QUEUE_SIZE` (32) entries when the config
//!   sets no `min_txq_size` (22000), 128 from AX210;
//! - a TFD queue's write pointer runs modulo `max_tfd_queue_size`: 256 on
//!   22000, 65536 from AX210 (`iwl_txq_inc_wrap`);
//! - a free receive entry is the buffer's address with its id in the low 12
//!   bits (`iwl_pcie_restock_bd`), so a 22000 buffer must be 4 KiB aligned
//!   (`supported_dma_mask`, 12 bits): the buffers are laid a page apart.

use super::super::plan::{CMD_RING, PAGE, RB_SIZE, RX_BUFS};
use super::CsrLayout;

/// `IWL_CMD_QUEUE_SIZE`.
const CMD_QUEUE_SIZE: usize = 32;

impl CsrLayout {
    /// Where a UMAC peripheral register sits above its base.
    pub const fn umac_offset(self) -> u32 {
        match self {
            CsrLayout::Gen2 => 0,
            CsrLayout::V1 | CsrLayout::V2 => 0x30_0000,
        }
    }

    /// The address bits the HBUS peripheral window decodes.
    pub const fn prph_mask(self) -> u32 {
        match self {
            CsrLayout::Gen2 => 0x000F_FFFF,
            CsrLayout::V1 | CsrLayout::V2 => 0x00FF_FFFF,
        }
    }

    /// The command ring's entries.
    pub const fn cmd_ring(self) -> usize {
        match self {
            CsrLayout::Gen2 => CMD_QUEUE_SIZE,
            CsrLayout::V1 | CsrLayout::V2 => CMD_RING,
        }
    }

    /// The mask a TFD queue's write pointer runs under.
    pub const fn wrap(self) -> u16 {
        match self {
            CsrLayout::Gen2 => 0xFF,
            CsrLayout::V1 | CsrLayout::V2 => 0xFFFF,
        }
    }

    /// How far apart the receive buffers lie in their region.
    pub const fn rb_stride(self) -> usize {
        match self {
            CsrLayout::Gen2 => PAGE,
            CsrLayout::V1 | CsrLayout::V2 => RB_SIZE,
        }
    }

    /// The receive buffers' region.
    pub const fn rb_region(self) -> usize {
        RX_BUFS * self.rb_stride()
    }
}
