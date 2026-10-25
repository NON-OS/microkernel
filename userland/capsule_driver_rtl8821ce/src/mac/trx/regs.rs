// NONOS Operating System (AGPL-3.0-or-later)

//! The transmit and receive engines' registers and bits.

// Register offsets (rtw88 reg.h).
pub(super) const REG_CR: usize = 0x0100;
pub(super) const REG_TXDMA_PQ_MAP: usize = 0x010C;
pub(super) const REG_RXFF_BNDY: usize = 0x011C;
pub(super) const REG_FIFOPAGE_CTRL_2: usize = 0x0204;
pub(super) const REG_AUTO_LLT_V1: usize = 0x0208;
pub(super) const REG_TXDMA_OFFSET_CHK: usize = 0x020C;
pub(super) const REG_RQPN_CTRL_2: usize = 0x022C;
pub(super) const REG_FIFOPAGE_INFO_1: usize = 0x0230;
pub(super) const REG_FIFOPAGE_INFO_2: usize = 0x0234;
pub(super) const REG_FIFOPAGE_INFO_3: usize = 0x0238;
pub(super) const REG_FIFOPAGE_INFO_4: usize = 0x023C;
pub(super) const REG_FIFOPAGE_INFO_5: usize = 0x0240;
pub(super) const REG_H2C_HEAD: usize = 0x0244;
pub(super) const REG_H2C_TAIL: usize = 0x0248;
pub(super) const REG_H2C_READ_ADDR: usize = 0x024C;
pub(super) const REG_H2C_INFO: usize = 0x0254;
pub(super) const REG_FWHW_TXQ_CTRL: usize = 0x0420;
pub(super) const REG_BCNQ_BDNY_V1: usize = 0x0424;
pub(super) const REG_BCNQ1_BDNY_V1: usize = 0x0456;
pub(super) const REG_H2CQ_CSR: usize = 0x1330;
pub(super) const REG_H2C_PKT_READADDR: usize = 0x10D0;
pub(super) const REG_H2C_PKT_WRITEADDR: usize = 0x10D4;

// PCI transmit/receive DMA control (rtw88 pci.h). Resetting the DMA interface and
// enabling the receive tag is what makes the card start advancing the receive
// write index; clearing the ring pointers and the H2C index gives it a clean
// start.
pub(super) const REG_PCI_CTRL: usize = 0x0300;
pub(super) const RST_TRXDMA_INTF: u32 = 1 << 20;
pub(super) const RX_TAG_EN: u32 = 1 << 15;
pub(super) const REG_TXBD_RWPTR_CLR: usize = 0x039C;
pub(super) const CLR_H2CQ_HOST_IDX: u32 = 1 << 16;
pub(super) const CLR_H2CQ_HW_IDX: u32 = 1 << 8;

// All eight engine-enable bits: HCI TX/RX DMA, TX/RX DMA, protocol, schedule,
// MAC TX/RX. Writing this to REG_CR is what turns the receiver on.
pub(super) const MAC_TRX_ENABLE: u8 = 0xFF;

// Latch the loaded reserved-queue page numbers.
pub(super) const LD_RQPN: u32 = 1 << 31;
