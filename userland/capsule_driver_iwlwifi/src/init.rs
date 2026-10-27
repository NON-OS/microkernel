// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use crate::constants::{
    ALL_INTS_MASK, CLOCK_READY_MS, CSR_FH_INT_STATUS, CSR_GP_CNTRL, CSR_INT, CSR_INT_COALESCING,
    CSR_INT_MASK, GP_CNTRL_INIT_DONE, GP_CNTRL_MAC_ACCESS_REQ, GP_CNTRL_MAC_CLOCK_READY,
    GP_CNTRL_XTAL_ON, INT_COALESCING_TIMEOUT, INT_MASK_DISABLED,
};
use crate::firmware::gen3::csr::CsrLayout;
use crate::regs::Regs;

/// Wake the chip in its family's CSR layout: V1 as every family up to AX210,
/// V2 (BZ) with MAC access and MAC_INIT asked and MAC_STATUS awaited, as
/// Linux's `iwl_finish_nic_init` does there.
pub fn bring_up(regs: Regs, csr: CsrLayout) -> Result<(), &'static str> {
    let ready = match csr {
        CsrLayout::V1 => {
            regs.set_bits(CSR_GP_CNTRL, GP_CNTRL_XTAL_ON);
            regs.set_bits(CSR_GP_CNTRL, GP_CNTRL_MAC_ACCESS_REQ | GP_CNTRL_INIT_DONE);
            GP_CNTRL_MAC_CLOCK_READY
        }
        CsrLayout::V2 => {
            let (bits, ready) = csr.init();
            regs.set_bits(CSR_GP_CNTRL, bits);
            ready
        }
    };
    if !regs.poll_set(CSR_GP_CNTRL, ready, CLOCK_READY_MS) {
        return Err("iwlwifi: mac clock not ready");
    }
    regs.write32(CSR_INT_COALESCING, INT_COALESCING_TIMEOUT);
    regs.write32(CSR_INT, ALL_INTS_MASK);
    regs.write32(CSR_INT_MASK, INT_MASK_DISABLED);
    regs.write32(CSR_FH_INT_STATUS, ALL_INTS_MASK);
    Ok(())
}
