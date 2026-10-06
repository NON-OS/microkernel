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

//! The registers the AX210-family (gen3) bring-up touches, and their bits.
//! Offsets and values are Linux v6.12 iwlwifi's (iwl-csr.h, iwl-fh.h,
//! iwl-prph.h), reimplemented as plain constants; the proofs pin the ones the
//! boot sequence depends on.

/// Hardware interface configuration.
pub const CSR_HW_IF_CONFIG_REG: usize = 0x000;
pub const HW_IF_CONFIG_HAP_WAKE_L1A: u32 = 0x0008_0000;
/// The host owns the NIC (PCI_OWN_SEM).
pub const HW_IF_CONFIG_NIC_READY: u32 = 0x0040_0000;
/// Ask the NIC to get ready (WAKE_ME).
pub const HW_IF_CONFIG_PREPARE: u32 = 0x0800_0000;

pub const CSR_INT_COALESCING: usize = 0x004;
/// Interrupt causes (legacy and MSI mode), write one to acknowledge.
pub const CSR_INT: usize = 0x008;
pub const CSR_INT_MASK: usize = 0x00C;
pub const CSR_FH_INT_STATUS: usize = 0x010;
pub const CSR_RESET: usize = 0x020;
pub const RESET_SW_RESET: u32 = 0x0000_0080;
pub const RESET_MASTER_DISABLED: u32 = 0x0000_0100;
pub const RESET_STOP_MASTER: u32 = 0x0000_0200;
pub const RESET_LINK_PWR_MGMT_DISABLED: u32 = 0x8000_0000;
pub const CSR_GP_CNTRL: usize = 0x024;
pub const CSR_HW_REV: usize = 0x028;
pub const CSR_GIO_REG: usize = 0x03C;
pub const GIO_L0S_DISABLED: u32 = 0x0000_0002;
pub const CSR_UCODE_DRV_GP1_CLR: usize = 0x05C;
pub const UCODE_SW_BIT_RFKILL: u32 = 0x0000_0002;
pub const UCODE_DRV_GP1_BIT_CMD_BLOCKED: u32 = 0x0000_0004;
pub const CSR_MBOX_SET_REG: usize = 0x088;
pub const MBOX_SET_OS_ALIVE: u32 = 1 << 5;
/// The RF identity (type, step, CDB) of the radio module.
pub const CSR_HW_RF_ID: usize = 0x09C;
pub const CSR_MAC_SHADOW_REG_CTRL: usize = 0x0A8;
pub const SHADOW_REG_ENABLE: u32 = 0x800F_FFFF;
pub const CSR_LTR_LAST_MSG: usize = 0x0DC;
pub const CSR_GIO_CHICKEN_BITS: usize = 0x100;
pub const GIO_CHICKEN_L1A_NO_L0S_RX: u32 = 0x0080_0000;
pub const CSR_DBG_HPET_MEM_REG: usize = 0x240;
pub const DBG_HPET_MEM_REG_VAL: u32 = 0xFFFF_0000;
pub const CSR_DBG_LINK_PWR_MGMT_REG: usize = 0x250;

/// CSR_GP_CNTRL bits (the v1 layout every family up to AX210 uses).
pub const GP_MAC_CLOCK_READY: u32 = 0x0000_0001;
pub const GP_INIT_DONE: u32 = 0x0000_0004;
pub const GP_MAC_ACCESS_REQ: u32 = 0x0000_0008;
pub const GP_GOING_TO_SLEEP: u32 = 0x0000_0010;
/// Set while the hardware RF-kill switch lets the radio on.
pub const GP_HW_RF_KILL_SW: u32 = 0x0800_0000;

/// CSR_INT causes.
pub const INT_BIT_ALIVE: u32 = 1 << 0;
pub const INT_BIT_SW_ERR: u32 = 1 << 25;
pub const INT_BIT_HW_ERR: u32 = 1 << 29;
pub const INT_BIT_FH_RX: u32 = 1 << 31;

/// MSI-X cause and mask registers (the device's own, beside the PCI table).
pub const CSR_MSIX_FH_INT_MASK_AD: usize = 0x2804;
pub const CSR_MSIX_HW_INT_CAUSES_AD: usize = 0x2808;
pub const CSR_MSIX_HW_INT_MASK_AD: usize = 0x280C;
pub const MSIX_HW_ALIVE: u32 = 1 << 0;
pub const MSIX_HW_IML: u32 = 1 << 1;
pub const MSIX_HW_SW_ERR: u32 = 1 << 25;
pub const MSIX_HW_HW_ERR: u32 = 1 << 29;

/// Peripheral (PRPH) registers reached through the HBUS window.
pub const HBUS_TARG_PRPH_WADDR: usize = 0x444;
pub const HBUS_TARG_PRPH_RADDR: usize = 0x448;
pub const HBUS_TARG_PRPH_WDAT: usize = 0x44C;
pub const HBUS_TARG_PRPH_RDAT: usize = 0x450;
/// Transmit queue write-pointer doorbell: `index | queue << 16`.
pub const HBUS_TARG_WRPTR: usize = 0x460;
/// Receive queue 0 free-buffer write index.
pub const RFH_Q0_FRBDCB_WIDX_TRG: usize = 0x1C80;

/// UMAC peripheral registers, written at `UMAC_PRPH_OFFSET` above their base.
pub const UREG_CPU_INIT_RUN: u32 = 0xA0_5C44;
pub const UREG_DOORBELL_TO_ISR6: u32 = 0xA0_5C04;
pub const UREG_DOORBELL_TO_ISR6_PNVM: u32 = 1 << 20;
pub const UMAG_SB_CPU_1_STATUS: u32 = 0xA0_38C0;
pub const UMAG_SB_CPU_2_STATUS: u32 = 0xA0_38C4;
