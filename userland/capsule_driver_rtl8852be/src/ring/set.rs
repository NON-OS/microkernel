// NONOS Operating System (AGPL-3.0-or-later)

//! Every ring the 8852B's DMA engine knows, and their programming into it:
//! `rtw89_pci_reset_trx_rings` (Linux v6.12 rtw89 `pci.c`). The data
//! (ACH0), management (CH8) and firmware command (CH12) rings carry frames;
//! ACH1-3 and CH9 are programmed onto small idle rings of their own, as the
//! engine expects every channel it serves to point at memory, and are never
//! kicked.

use super::addrs::{bdram_word, ChAddrs, ACH1, ACH2, ACH3, CH9};
use super::rx::RxRing;
use super::tx::TxRing;
use crate::dma::DmaMem;
use crate::regs::Mmio;

pub const ACH0_LEN: u32 = 32;
pub const CH8_LEN: u32 = 16;
pub const CH12_LEN: u32 = 32;
pub const RXQ_LEN: u32 = 32;
pub const RPQ_LEN: u32 = 16;
/// A receive buffer: rtw89's `RTW89_PCI_RX_BUF_SIZE` (11454 + 40), rounded
/// up to whole pages.
pub const RXQ_BUF: usize = 12288;
pub const RPQ_BUF: usize = 4096;
/// The idle rings' length, each in its own 64 bytes of one page.
pub const IDLE_LEN: u32 = 8;
pub const IDLE: [ChAddrs; 4] = [ACH1, ACH2, ACH3, CH9];

pub struct Rings<M: DmaMem> {
    pub ach0: TxRing<M>,
    pub ch8: TxRing<M>,
    pub ch12: TxRing<M>,
    pub idle: M,
    pub rxq: RxRing<M>,
    pub rpq: RxRing<M>,
}

impl<M: DmaMem> Rings<M> {
    /// Point the engine at every ring and zero every pointer.
    pub fn program(&mut self, r: &impl Mmio) {
        for t in [&mut self.ach0, &mut self.ch8, &mut self.ch12] {
            t.wp = 0;
            t.rp = 0;
            t.busy = 0;
            tx_regs(r, &t.ch, t.len, t.mem.device_addr() as u32);
        }
        for (i, ch) in IDLE.iter().enumerate() {
            tx_regs(r, ch, IDLE_LEN, self.idle.device_addr() as u32 + i as u32 * 64);
        }
        for q in [&mut self.rxq, &mut self.rpq] {
            q.wp = 0;
            let (num, _, desa_l) = q.regs;
            r.write16(num, q.len as u16);
            r.write32(desa_l, q.mem.device_addr() as u32);
            r.write32(desa_l + 4, 0);
        }
    }
}

fn tx_regs(r: &impl Mmio, ch: &ChAddrs, len: u32, bus: u32) {
    r.write16(ch.num, len as u16);
    r.write32(ch.bdram, bdram_word(ch.ram));
    r.write32(ch.desa_l, bus);
    r.write32(ch.desa_l + 4, 0);
}
