// NONOS Operating System (AGPL-3.0-or-later)

//! The release report and the WD page's WP info and address info, `pci.h`
//! in Linux v6.12 rtw89 (the 8-byte address mode
//! `rtw89_pci_ops_mac_post_init_ax` selects).

/// `RTW89_PCI_TXWP_VALID`, `RTW89_PCI_ADDR_MSDU_LS`.
pub const TXWP_VALID: u16 = 1 << 15;
pub const ADDR_MSDU_LS: u16 = 1 << 15;
pub const WP_INFO_SIZE: usize = 8;
pub const ADDR_INFO_SIZE: usize = 8;

/// A release report: the WD page's sequence, the transmit status, the
/// queue it went out of.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rpp {
    pub seq: u16,
    pub status: u8,
    pub qsel: u8,
}

pub fn rpp(word: u32) -> Rpp {
    Rpp {
        seq: ((word >> 16) & 0x7fff) as u16,
        status: ((word >> 13) & 7) as u8,
        qsel: ((word >> 8) & 0x1f) as u8,
    }
}

/// The WD page's WP info and one address info entry, after the WD:
/// `rtw89_pci_txwd_submit` and `rtw89_pci_fill_txaddr_info`.
pub fn wp_and_addr(seq: u16, len: u16, dma: u32) -> [u8; WP_INFO_SIZE + ADDR_INFO_SIZE] {
    let mut b = [0u8; WP_INFO_SIZE + ADDR_INFO_SIZE];
    b[..2].copy_from_slice(&(seq | TXWP_VALID).to_le_bytes());
    b[8..10].copy_from_slice(&len.to_le_bytes());
    b[10..12].copy_from_slice(&(ADDR_MSDU_LS | 1).to_le_bytes());
    b[12..].copy_from_slice(&dma.to_le_bytes());
    b
}
