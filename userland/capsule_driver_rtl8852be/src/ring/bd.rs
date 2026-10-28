// NONOS Operating System (AGPL-3.0-or-later)

//! The PCI descriptors, `pci.h` in Linux v6.12 rtw89: the 8-byte transmit
//! and receive buffer descriptors, the receive buffer's leading info word
//! (the release report and the WD page's tail are in `wd`). Every region
//! lies below 4 GiB, so each DMA_HI field is written zero.

pub const TX_BD_SIZE: usize = 8;
pub const RX_BD_SIZE: usize = 8;
/// `RTW89_PCI_TXBD_OPT_LS`.
pub const TXBD_OPT_LS: u16 = 1 << 14;
pub const RXBD_FS: u32 = 1 << 15;
pub const RXBD_LS: u32 = 1 << 14;
pub const RXBD_WRITE_SIZE: u32 = 0x3fff;
/// The info word at the head of every receive buffer.
pub const RXBD_INFO_SIZE: usize = 4;

/// A transmit buffer descriptor for `len` bytes at bus address `dma`.
pub fn tx_bd(len: u16, dma: u32) -> [u8; TX_BD_SIZE] {
    let mut b = [0u8; TX_BD_SIZE];
    b[..2].copy_from_slice(&len.to_le_bytes());
    b[2..4].copy_from_slice(&TXBD_OPT_LS.to_le_bytes());
    b[4..].copy_from_slice(&dma.to_le_bytes());
    b
}

/// A receive buffer descriptor handing the card `size` bytes at `dma`.
pub fn rx_bd(size: u16, dma: u32) -> [u8; RX_BD_SIZE] {
    let mut b = [0u8; RX_BD_SIZE];
    b[..2].copy_from_slice(&size.to_le_bytes());
    b[4..].copy_from_slice(&dma.to_le_bytes());
    b
}

/// The receive buffer's info word: first and last segment, bytes written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RxInfo {
    pub fs: bool,
    pub ls: bool,
    pub len: u32,
}

pub fn rx_info(word: u32) -> RxInfo {
    RxInfo { fs: word & RXBD_FS != 0, ls: word & RXBD_LS != 0, len: word & RXBD_WRITE_SIZE }
}
