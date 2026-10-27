// NONOS Operating System (AGPL-3.0-or-later)

//! The CSR bit layout a family uses. Linux v6.12 moves several bits for
//! `IWL_DEVICE_FAMILY_BZ` (iwl-csr.h): D0A is asked with MAC_INIT (6) and
//! read from MAC_STATUS (20); MAC access is asked with bit 21; the software
//! reset (31) and the bus-master stop (29, status 28) move from CSR_RESET to
//! CSR_GP_CNTRL; the ROM is started by ROM_START (7) after CSR_FUNC_SCRATCH,
//! not by UREG_CPU_INIT_RUN; the MSI-X software error is bit 5, not 25; the
//! receive free index goes through HBUS_TARG_WRPTR, a used-ring entry is four
//! bytes and a byte count table holds 4096 entries. `V1` is every family up
//! to AX210 (SO, TY, MA); `V2` is BZ (the Gl and Bz MACs).

use super::super::regs::MSIX_HW_SW_ERR;
use super::super::regs::{GP_GOING_TO_SLEEP, GP_INIT_DONE, GP_MAC_ACCESS_REQ, GP_MAC_CLOCK_READY};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CsrLayout {
    V1,
    V2,
}

pub const GP_MAC_INIT: u32 = 1 << 6;
pub const GP_ROM_START: u32 = 1 << 7;
pub const GP_MAC_STATUS: u32 = 1 << 20;
pub const GP_BZ_MAC_ACCESS_REQ: u32 = 1 << 21;
pub const GP_BUS_MASTER_DISABLE_STATUS: u32 = 1 << 28;
pub const GP_BUS_MASTER_DISABLE_REQ: u32 = 1 << 29;
pub const GP_SW_RESET: u32 = 1 << 31;
/// `CSR_FUNC_SCRATCH` and `CSR_FUNC_SCRATCH_INIT_VALUE`.
pub const CSR_FUNC_SCRATCH: usize = 0x02C;
pub const FUNC_SCRATCH_INIT: u32 = 0x0101_0101;
/// `MSIX_HW_INT_CAUSES_REG_SW_ERR_BZ`.
pub const MSIX_HW_SW_ERR_BZ: u32 = 1 << 5;
/// `HBUS_TARG_WRPTR_RX_Q(0)`: receive queue 0 in the doorbell's queue field.
pub const WRPTR_RX_Q0: u32 = 512 << 16;

impl CsrLayout {
    /// What `iwl_finish_nic_init` sets, and the bit it then waits for.
    pub const fn init(self) -> (u32, u32) {
        match self {
            CsrLayout::V1 => (GP_INIT_DONE, GP_MAC_CLOCK_READY),
            CsrLayout::V2 => (GP_BZ_MAC_ACCESS_REQ | GP_MAC_INIT, GP_MAC_STATUS),
        }
    }

    /// `__iwl_trans_pcie_grab_nic_access`: the request bit, the mask read
    /// back and the value it must have.
    pub const fn access(self) -> (u32, u32, u32) {
        match self {
            CsrLayout::V1 => {
                (GP_MAC_ACCESS_REQ, GP_MAC_CLOCK_READY | GP_GOING_TO_SLEEP, GP_MAC_CLOCK_READY)
            }
            CsrLayout::V2 => (GP_BZ_MAC_ACCESS_REQ, GP_MAC_STATUS, GP_MAC_STATUS),
        }
    }

    /// The MSI-X cause the firmware's software error raises.
    pub const fn msix_sw_err(self) -> u32 {
        match self {
            CsrLayout::V1 => MSIX_HW_SW_ERR,
            CsrLayout::V2 => MSIX_HW_SW_ERR_BZ,
        }
    }
}
