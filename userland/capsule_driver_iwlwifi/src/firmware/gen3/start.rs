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

//! The register sequence that takes an AX210-family part from claimed to
//! running its boot ROM, in Linux v6.12 order: `_iwl_trans_pcie_start_hw`
//! (prepare the card, software reset and retake ownership, APM init), then
//! `iwl_trans_pcie_gen2_start_fw` up to the self-load kick (prepare again,
//! clear and mask interrupts, clear the RF-kill handshake bits, gen2 APM init,
//! interrupt coalescing, shadow registers), and after the kick the LTR
//! workaround, `UREG_CPU_INIT_RUN` and the wait for the image loader. Every
//! wait is bounded; the ones Linux treats as fatal are fatal here too.
//!
//! Interrupts stay with the cause registers: the driver polls them. Linux
//! programs either the legacy mask or the MSI-X masks depending on how the
//! PCI function's interrupt was set up; this driver does not know which the
//! broker enabled, so it programs both and polls both cause registers.

use super::prph::write_umac;
use super::region::Clock;
use super::regs::*;
use crate::regs::Mmio;

/// Why the chip did not start.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StartError {
    /// The NIC never reported ready (`iwl_pcie_prepare_card_hw`); another
    /// owner (the CSME management engine) may hold it.
    NotReady,
    /// The MAC clock did not stabilise (`iwl_finish_nic_init`).
    ClockNotReady,
    /// The MAC would not wake for a peripheral register write.
    NoNicAccess,
}

const READY_POLL_MS: u32 = 1;
const PREPARE_ROUNDS: u32 = 10;
const PREPARE_POLL_MS: u32 = 150;
const CLOCK_POLL_MS: u32 = 25;
const IML_POLL_MS: u32 = 100;
const STOP_MASTER_MS: u32 = 1;
/// `IWL_HOST_INT_TIMEOUT_DEF`.
const INT_COALESCING_DEF: u32 = 0x40;

fn set<M: Mmio>(m: &M, reg: usize, bits: u32) {
    m.write32(reg, m.read32(reg) | bits);
}

// `iwl_pcie_set_hw_ready`.
fn set_hw_ready<M: Mmio, C: Clock>(m: &M, c: &mut C) -> bool {
    set(m, CSR_HW_IF_CONFIG_REG, HW_IF_CONFIG_NIC_READY);
    let ready = c.poll_for(READY_POLL_MS, &mut || {
        m.read32(CSR_HW_IF_CONFIG_REG) & HW_IF_CONFIG_NIC_READY != 0
    });
    if ready {
        set(m, CSR_MBOX_SET_REG, MBOX_SET_OS_ALIVE);
    }
    ready
}

/// `iwl_pcie_prepare_card_hw`: take ownership of the NIC.
pub fn prepare_card_hw<M: Mmio, C: Clock>(m: &M, c: &mut C) -> Result<(), StartError> {
    if set_hw_ready(m, c) {
        return Ok(());
    }
    set(m, CSR_DBG_LINK_PWR_MGMT_REG, RESET_LINK_PWR_MGMT_DISABLED);
    c.delay_us(1000);
    for _ in 0..PREPARE_ROUNDS {
        set(m, CSR_HW_IF_CONFIG_REG, HW_IF_CONFIG_PREPARE);
        let mut attempt = || set_hw_ready_once(m);
        if c.poll_for(PREPARE_POLL_MS, &mut attempt) {
            set(m, CSR_MBOX_SET_REG, MBOX_SET_OS_ALIVE);
            return Ok(());
        }
        c.delay_us(25_000);
    }
    Err(StartError::NotReady)
}

// One `set_hw_ready` probe without its own wait (the caller polls).
fn set_hw_ready_once<M: Mmio>(m: &M) -> bool {
    set(m, CSR_HW_IF_CONFIG_REG, HW_IF_CONFIG_NIC_READY);
    m.read32(CSR_HW_IF_CONFIG_REG) & HW_IF_CONFIG_NIC_READY != 0
}

/// `iwl_finish_nic_init`: move the adapter to D0A and wait for the clock.
fn finish_nic_init<M: Mmio, C: Clock>(m: &M, c: &mut C) -> Result<(), StartError> {
    set(m, CSR_GP_CNTRL, GP_INIT_DONE);
    let up = c.poll_for(CLOCK_POLL_MS, &mut || m.read32(CSR_GP_CNTRL) & GP_MAC_CLOCK_READY != 0);
    up.then_some(()).ok_or(StartError::ClockNotReady)
}

/// The APM init both `iwl_pcie_apm_init` and `iwl_pcie_gen2_apm_init` run on
/// this family: L0s off, FH wait threshold, HAP wake, then D0A.
fn apm_init<M: Mmio, C: Clock>(m: &M, c: &mut C) -> Result<(), StartError> {
    set(m, CSR_GIO_CHICKEN_BITS, GIO_CHICKEN_L1A_NO_L0S_RX);
    set(m, CSR_DBG_HPET_MEM_REG, DBG_HPET_MEM_REG_VAL);
    set(m, CSR_HW_IF_CONFIG_REG, HW_IF_CONFIG_HAP_WAKE_L1A);
    set(m, CSR_GIO_REG, GIO_L0S_DISABLED);
    finish_nic_init(m, c)
}

/// `_iwl_trans_pcie_start_hw`: prepare, software reset (which drops
/// ownership, so prepare again), APM init.
pub fn start_hw<M: Mmio, C: Clock>(m: &M, c: &mut C) -> Result<(), StartError> {
    prepare_card_hw(m, c)?;
    set(m, CSR_RESET, RESET_SW_RESET);
    c.delay_us(6000);
    prepare_card_hw(m, c)?;
    apm_init(m, c)
}

/// Whether the hardware RF-kill switch has the radio off.
pub fn rf_killed<M: Mmio>(m: &M) -> bool {
    m.read32(CSR_GP_CNTRL) & GP_HW_RF_KILL_SW == 0
}

fn mask_all<M: Mmio>(m: &M) {
    m.write32(CSR_INT_MASK, 0);
    m.write32(CSR_INT, 0xFFFF_FFFF);
    m.write32(CSR_FH_INT_STATUS, 0xFFFF_FFFF);
    m.write32(CSR_MSIX_FH_INT_MASK_AD, 0xFFFF_FFFF);
    m.write32(CSR_MSIX_HW_INT_MASK_AD, 0xFFFF_FFFF);
}

/// Stop the device signalling interrupts once ALIVE is in: this driver
/// polls the receive index and the cause registers, which still latch with
/// every mask set, and an INTx line left asserted could be shared with
/// another device. The cause bits are left for the error checks to read.
pub fn mask_interrupts<M: Mmio>(m: &M) {
    m.write32(CSR_INT_MASK, 0);
    m.write32(CSR_MSIX_FH_INT_MASK_AD, 0xFFFF_FFFF);
    m.write32(CSR_MSIX_HW_INT_MASK_AD, 0xFFFF_FFFF);
}

/// `iwl_trans_pcie_gen2_start_fw` up to the point the context info is
/// handed over: the caller lays out memory and kicks the self-load next.
pub fn before_kick<M: Mmio, C: Clock>(m: &M, c: &mut C) -> Result<(), StartError> {
    prepare_card_hw(m, c)?;
    m.write32(CSR_INT, 0xFFFF_FFFF);
    mask_all(m);
    m.write32(CSR_UCODE_DRV_GP1_CLR, UCODE_SW_BIT_RFKILL);
    m.write32(CSR_UCODE_DRV_GP1_CLR, UCODE_DRV_GP1_BIT_CMD_BLOCKED);
    m.write32(CSR_INT, 0xFFFF_FFFF);
    apm_init(m, c)?;
    m.write32(CSR_INT_COALESCING, INT_COALESCING_DEF);
    set(m, CSR_MAC_SHADOW_REG_CTRL, SHADOW_REG_ENABLE);
    Ok(())
}

/// `iwl_enable_fw_load_int_ctx_info`, for both interrupt modes: the ALIVE
/// cause and receive.
pub fn enable_alive_int<M: Mmio>(m: &M) {
    m.write32(CSR_INT_MASK, INT_BIT_ALIVE | INT_BIT_FH_RX);
    m.write32(CSR_MSIX_HW_INT_MASK_AD, !MSIX_HW_ALIVE);
    m.write32(CSR_MSIX_FH_INT_MASK_AD, 0);
}

/// After the kick: the integrated-part LTR workaround (clear the image loader
/// cause, then keep the device busy until it is set), and starting the CPU.
pub fn after_kick<M: Mmio, C: Clock>(m: &M, c: &mut C) -> Result<(), StartError> {
    m.write32(CSR_MSIX_HW_INT_CAUSES_AD, MSIX_HW_IML);
    if !write_umac(m, c, UREG_CPU_INIT_RUN, 1) {
        return Err(StartError::NoNicAccess);
    }
    // `iwl_pcie_spin_for_iml`: not fatal; the ALIVE wait decides.
    let _ = c.poll_for(IML_POLL_MS, &mut || {
        let _ = m.read32(CSR_LTR_LAST_MSG);
        m.read32(CSR_MSIX_HW_INT_CAUSES_AD) & MSIX_HW_IML != 0
    });
    Ok(())
}

/// What a failed bring-up leaves the card at (`iwl_trans_pcie_gen2_stop_device`
/// reduced to the part that quiets it): every interrupt masked, bus mastering
/// stopped (`iwl_pcie_apm_stop_master`; a timeout is not fatal, as in Linux),
/// then a software reset that halts the firmware. `false` when the master
/// never reported stopped.
pub fn stop_device<M: Mmio, C: Clock>(m: &M, c: &mut C) -> bool {
    mask_all(m);
    set(m, CSR_RESET, RESET_STOP_MASTER);
    let stopped = c.poll_for(STOP_MASTER_MS, &mut || m.read32(CSR_RESET) & RESET_MASTER_DISABLED != 0);
    set(m, CSR_RESET, RESET_SW_RESET);
    c.delay_us(6000);
    stopped
}
