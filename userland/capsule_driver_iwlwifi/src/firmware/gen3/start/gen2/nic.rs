// NONOS Operating System (AGPL-3.0-or-later)

//! After APM init the radio configuration goes into CSR_HW_IF_CONFIG_REG,
//! as mvm's `iwl_mvm_nic_config` does before AX210: the MAC step and dash
//! from CSR_HW_REV, the radio type, step and dash from the image's PHY
//! config, the register's other bits kept.

use super::super::super::regs::CSR_HW_IF_CONFIG_REG;
use crate::regs::Mmio;

/// What `iwl_mvm_nic_config` owns of CSR_HW_IF_CONFIG_REG: MAC step and
/// dash, PHY type, dash and step, MAC_SI and RADIO_SI (D3_DEBUG shares 9).
pub const NIC_CONFIG_MASK: u32 = 0x0000_FF0F;

/// `iwl_mvm_nic_config`'s value for CSR_HW_IF_CONFIG_REG's owned bits.
pub fn nic_config(hw_rev: u32, phy_config: u32) -> u32 {
    let (ty, step, dash) = (phy_config & 3, (phy_config >> 2) & 3, (phy_config >> 4) & 3);
    (hw_rev & 0xF) | ty << 10 | step << 14 | dash << 12
}

/// Write it, keeping the register's other bits.
pub fn set_nic_config<M: Mmio>(m: &M, hw_rev: u32, phy_config: u32) {
    let keep = m.read32(CSR_HW_IF_CONFIG_REG) & !NIC_CONFIG_MASK;
    m.write32(CSR_HW_IF_CONFIG_REG, keep | nic_config(hw_rev, phy_config));
}
