// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use crate::constants::{
    ALL_INTS_MASK, CLOCK_READY_MS, CSR_FH_INT_STATUS, CSR_GP_CNTRL, CSR_INT, CSR_INT_COALESCING,
    CSR_INT_MASK, INT_COALESCING_TIMEOUT, INT_MASK_DISABLED,
};
use crate::firmware::gen3::csr::CsrLayout;
use crate::firmware::gen3::region::Clock;
use crate::firmware::gen3::start::prepare_card_hw;
use crate::regs::Regs;

/// Wake the chip as Linux v6.12's probe does (`iwl_pci_probe`): take the
/// NIC first (`iwl_pcie_prepare_card_hw`: NIC_READY, else PREPARE until it
/// is, which takes a card back from a previous owner), and only then ask for
/// D0A in the family's CSR layout and wait for the MAC clock
/// (`iwl_finish_nic_init`: INIT_DONE and MAC_CLOCK_READY up to AX210,
/// MAC access and MAC_INIT then MAC_STATUS on BZ). A NIC that never reports
/// ready is left for the start to name, as Linux's probe goes on without it.
pub fn bring_up<C: Clock>(regs: Regs, csr: CsrLayout, c: &mut C) -> Result<(), &'static str> {
    if prepare_card_hw(&regs, c).is_ok() {
        let (bits, ready) = csr.init();
        regs.set_bits(CSR_GP_CNTRL, bits);
        if !regs.poll_set(CSR_GP_CNTRL, ready, CLOCK_READY_MS) {
            return Err("iwlwifi: mac clock not ready");
        }
    }
    regs.write32(CSR_INT_COALESCING, INT_COALESCING_TIMEOUT);
    regs.write32(CSR_INT, ALL_INTS_MASK);
    regs.write32(CSR_INT_MASK, INT_MASK_DISABLED);
    regs.write32(CSR_FH_INT_STATUS, ALL_INTS_MASK);
    Ok(())
}
