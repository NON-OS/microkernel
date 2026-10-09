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

//! PCI device IDs, one list per board Linux binds them to in e1000e
//! netdev.c `e1000_pci_tbl`. Each ID is the hw.h `E1000_DEV_ID_*` macro
//! named in the list's comment, in the order hw.h defines them.

/// board_82574: 82574L (QEMU's `-device e1000e`), 82574LA.
pub const I82574: &[u16] = &[0x10D3, 0x10F6];
/// board_82583: 82583V.
pub const I82583: &[u16] = &[0x150C];
/// board_pch_lpt: PCH_LPT_I217_LM, _V, PCH_LPTLP_I218_LM, _V,
/// PCH_I218_LM2, _V2, PCH_I218_LM3, _V3.
pub const PCH_LPT: &[u16] = &[0x153A, 0x153B, 0x155A, 0x1559, 0x15A0, 0x15A1, 0x15A2, 0x15A3];
/// board_pch_spt: PCH_SPT_I219_LM, _V, _LM2, _V2, PCH_LBG_I219_LM3,
/// PCH_SPT_I219_LM4, _V4, _LM5, _V5, and PCH_CMP_I219_LM12, _V12, which
/// netdev.c binds to the SPT board although hw.h files them under CMP.
pub const PCH_SPT: &[u16] =
    &[0x156F, 0x1570, 0x15B7, 0x15B8, 0x15B9, 0x15D7, 0x15D8, 0x15E3, 0x15D6, 0x0D53, 0x0D55];
/// board_pch_cnp: PCH_CNP_I219_LM6, _V6, _LM7, _V7, PCH_ICP_I219_LM8, _V8,
/// _LM9, _V9, PCH_CMP_I219_LM10, _V10, _LM11, _V11.
pub const PCH_CNP: &[u16] = &[
    0x15BD, 0x15BE, 0x15BB, 0x15BC, 0x15DF, 0x15E0, 0x15E1, 0x15E2, 0x0D4E, 0x0D4F, 0x0D4C, 0x0D4D,
];
/// board_pch_tgp: PCH_TGP_I219_LM13, _V13, _LM14, _V14, _LM15, _V15.
pub const PCH_TGP: &[u16] = &[0x15FB, 0x15FC, 0x15F9, 0x15FA, 0x15F4, 0x15F5];
/// board_pch_adp: PCH_RPL_I219_LM23, _V23, PCH_ADP_I219_LM16, _V16, _LM17,
/// _V17, PCH_RPL_I219_LM22, _V22, PCH_ADP_I219_LM19, _V19.
pub const PCH_ADP: &[u16] =
    &[0x0DC5, 0x0DC6, 0x1A1E, 0x1A1F, 0x1A1C, 0x1A1D, 0x0DC7, 0x0DC8, 0x550C, 0x550D];
/// board_pch_mtp: PCH_MTP_I219_LM18, _V18, PCH_LNP_I219_LM20, _V20, _LM21,
/// _V21, PCH_ARL_I219_LM24, _V24. hw.h has an e1000_pch_lnp MAC type, but no
/// board sets it: netdev.c binds the LNP IDs to board_pch_mtp, so they run
/// as pch_mtp in Linux and here.
pub const PCH_MTP: &[u16] = &[0x550A, 0x550B, 0x550E, 0x550F, 0x5510, 0x5511, 0x57A0, 0x57A1];
/// board_pch_ptp: PCH_PTP_I219_LM25, _V25, _LM27, _V27, PCH_NVL_I219_LM29,
/// _V29.
pub const PCH_PTP: &[u16] = &[0x57B3, 0x57B4, 0x57B7, 0x57B8, 0x57B9, 0x57BA];

/// e1000_disable_ulp_lpt_lp returns at once for these four: PCH_LPT_I217_LM,
/// _V and PCH_I218_LM2, _V2 have no ULP to leave.
pub const NO_ULP: &[u16] = &[0x153A, 0x153B, 0x15A0, 0x15A1];
