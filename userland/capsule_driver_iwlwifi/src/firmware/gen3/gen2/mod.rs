// NONOS Operating System (AGPL-3.0-or-later)

//! The 22000 family's boot (Qu, QuZ and Cc MACs: the AX201 CNVi of Ice,
//! Comet, Tiger and Jasper Lake, and the discrete AX200), as Linux v6.12
//! pcie/ctxt-info.c `iwl_pcie_ctxt_info_init` and pcie/trans-gen2.c
//! `iwl_trans_pcie_gen2_start_fw` run it before AX210. The ROM is handed the
//! version 1 context info (`ctxt_info`), not the gen3 one and its peripheral
//! scratch: the receive rings, the command ring and the image maps in one
//! structure, its address in CSR_CTXT_INFO_BA, and no image loader. The
//! start, the LTR and the CPU kick differ as `start::gen2` and `start::cpu`
//! say; ALIVE and everything after are the gen3 path's, in the rings and
//! packets `csr::gen2`, `rxq` and `dev::gen2` lay out for this family.

pub mod boot;
pub mod ctxt_info;
pub mod layout;

pub use boot::start_and_kick;
