// NONOS Operating System (AGPL-3.0-or-later)

//! The transport values per PCI device: cfg/ax210.c's `iwl_so*_mac_cfg`,
//! `iwl_ty_mac_cfg`, `iwl_ma_mac_cfg`, and cfg/bz.c's `iwl_bz_trans_cfg` and
//! `iwl_gl_trans_cfg`, with pcie/drv.c `iwl_hw_card_ids` naming the devices.

use super::super::csr::CsrLayout;
use super::Transport;

const V1: Transport = Transport {
    integrated: true,
    ltr_delay: 0,
    xtal_latency: 0,
    low_latency_xtal: false,
    imr_enabled: false,
    csr: CsrLayout::V1,
};
const SO: Transport = Transport { ltr_delay: 1, xtal_latency: 500, ..V1 };
const SO_LONG: Transport =
    Transport { ltr_delay: 2, xtal_latency: 12000, low_latency_xtal: true, ..V1 };
const SO_LONG_IMR: Transport = Transport { imr_enabled: true, ..SO_LONG };
/// `iwl_ty_mac_cfg`: a discrete card, no LTR delay.
const TY: Transport = Transport { integrated: false, xtal_latency: 500, ..V1 };
/// `iwl_ma_mac_cfg`: integrated, with no LTR delay or crystal latency set.
const MA: Transport = V1;
/// `iwl_bz_trans_cfg`: the CNVi, integrated, LTR delay 2500 us.
const BZ: Transport = Transport { csr: CsrLayout::V2, ..SO_LONG };
/// `iwl_gl_trans_cfg`: the discrete BE200, no LTR delay.
const GL: Transport = Transport { integrated: false, ltr_delay: 0, ..BZ };
/// cfg/22000.c `iwl_qu_trans_cfg`: the CNVi, crystal 500, LTR delay 200 us;
/// `iwl_qu_medium_latency_trans_cfg`: 1820 and 1820 us (3);
/// `iwl_qu_long_latency_trans_cfg`: 12000, low latency, 2500 us;
/// `iwl_ax200_trans_cfg`: discrete, nothing set.
const QU: Transport = Transport { csr: CsrLayout::Gen2, ..SO };
const QU_MEDIUM: Transport = Transport { ltr_delay: 3, xtal_latency: 1820, ..QU };
const QU_LONG: Transport = Transport { csr: CsrLayout::Gen2, ..SO_LONG };
const AX200: Transport = Transport { integrated: false, csr: CsrLayout::Gen2, ..V1 };

/// The transport for a PCI device ID, or `None` for one outside the 22000,
/// AX210 and BZ families. Linux takes 0xA840 only with listed subsystem
/// ids; any is taken here, the MAC and RF then deciding.
pub fn transport(pci_device: u16) -> Option<Transport> {
    match pci_device {
        0x7AF0 | 0x7F70 => Some(SO),
        0x51F0 | 0x54F0 => Some(SO_LONG),
        0x7A70 | 0x51F1 => Some(SO_LONG_IMR),
        0x2725 => Some(TY),
        0x2729 | 0x7E40 => Some(MA),
        0x272B => Some(GL),
        0xA840 | 0x7740 | 0x4D40 => Some(BZ),
        0x02F0 | 0x06F0 => Some(QU),
        0x34F0 | 0x3DF0 | 0x4DF0 => Some(QU_MEDIUM),
        0x43F0 | 0xA0F0 => Some(QU_LONG),
        0x2723 => Some(AX200),
        _ => None,
    }
}
