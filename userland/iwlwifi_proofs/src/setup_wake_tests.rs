// NONOS Operating System (AGPL-3.0-or-later)

//! Setup's wake is Linux v6.12's probe wake (`iwl_pci_probe`): the NIC
//! taken first (`iwl_pcie_prepare_card_hw`: NIC_READY seen, OS_ALIVE told),
//! then exactly the D0A request `iwl_finish_nic_init` makes in the family's
//! CSR layout, nothing else in CSR_GP_CNTRL (no XTAL_ON, no MAC access
//! request left set on V1).

use crate::constants::CSR_GP_CNTRL;
use crate::gen3::csr::{CsrLayout, GP_BZ_MAC_ACCESS_REQ, GP_MAC_INIT, GP_MAC_STATUS};
use crate::gen3::regs::{CSR_HW_IF_CONFIG_REG, CSR_MBOX_SET_REG, GP_INIT_DONE};
use crate::gen3::regs::{GP_MAC_CLOCK_READY, HW_IF_CONFIG_NIC_READY, MBOX_SET_OS_ALIVE};
use crate::gen3_model::Polls;
use crate::grants::{init_or_release, Grants};
use crate::regs::Regs;

fn wake(csr: CsrLayout, ready: u32) -> Vec<u32> {
    let mut w = vec![0u32; 0x100 / 4];
    w[CSR_GP_CNTRL / 4] = ready;
    // SAFETY: the window's own 0x100 bytes, alive for the call.
    let regs = unsafe { Regs::new(w.as_mut_ptr() as u64, 0x100) };
    let mut g = Grants { device_id: 5, claim: Some(1), mmio: Some(6), irq: Some(7) };
    let mut c = Polls { per_wait: 64, delays_us: 0 };
    assert_eq!(init_or_release(regs, csr, &mut c, &mut g), Ok(()));
    w
}

#[test]
fn the_wake_takes_the_nic_then_asks_for_d0a_in_the_family_layout() {
    for csr in [CsrLayout::V1, CsrLayout::Gen2] {
        let w = wake(csr, GP_MAC_CLOCK_READY);
        assert_eq!(w[CSR_HW_IF_CONFIG_REG / 4] & HW_IF_CONFIG_NIC_READY, HW_IF_CONFIG_NIC_READY);
        assert_eq!(w[CSR_MBOX_SET_REG / 4], MBOX_SET_OS_ALIVE, "{csr:?}: the NIC taken");
        assert_eq!(w[CSR_GP_CNTRL / 4], GP_MAC_CLOCK_READY | GP_INIT_DONE, "{csr:?}");
    }
    let w = wake(CsrLayout::V2, GP_MAC_STATUS);
    assert_eq!(w[CSR_MBOX_SET_REG / 4], MBOX_SET_OS_ALIVE, "V2: the NIC taken");
    assert_eq!(w[CSR_GP_CNTRL / 4], GP_MAC_STATUS | GP_MAC_INIT | GP_BZ_MAC_ACCESS_REQ);
}
