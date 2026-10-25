// NONOS Operating System (AGPL-3.0-or-later)

//! What the firmware needs before the station may sleep, given on association
//! as rtw88 gives it (`rtw_ops_bss_info_changed`): the reserved pages and where
//! they are, the AID and DTIM period, and the station reported connected.

use super::download::download;
use super::pages::{sta_pages, NULL_PAGE, PS_POLL_PAGE, QOS_NULL_PAGE};
use super::state::Ps;
use super::switch::LPS;
use crate::fw::dma::DmaMem;
use crate::h2c::cmds::{media_status, rsvd_page};
use crate::regs::Mmio;

/// Port 0's AID field (rtw88 `rtw_vif_port`), and the DTIM count's root.
const REG_BCN_PSR_RPT: usize = 0x06A8;
const AID_MASK: u32 = 0x07FF;
const REG_DTIM_COUNTER_ROOT: usize = 0x05A8;

/// The association power save is told of.
#[derive(Clone, Copy)]
pub struct Bss {
    pub aid: u16,
    pub dtim: u8,
    pub bssid: [u8; 6],
    pub sta: [u8; 6],
}

impl Ps {
    /// Nothing while the switch is off.
    pub fn joined<M: Mmio, B: DmaMem>(&mut self, mmio: &M, beacon: Option<(&B, &B)>, bss: Bss) {
        if LPS {
            self.tell_joined(mmio, beacon, bss);
        }
    }

    /// `joined` regardless of the switch. The station may sleep only once the
    /// pages landed and the firmware took both commands.
    pub fn tell_joined<M: Mmio, B: DmaMem>(
        &mut self,
        mmio: &M,
        beacon: Option<(&B, &B)>,
        bss: Bss,
    ) {
        let (w0, w1) = media_status(true, 0);
        let connected = self.h2c.send(mmio, w0, w1);
        let landed = beacon.is_some_and(|(stage, ring)| {
            download(mmio, stage, ring, &sta_pages(bss.aid, bss.bssid, bss.sta))
        });
        let (w0, w1) = rsvd_page(PS_POLL_PAGE, NULL_PAGE, QOS_NULL_PAGE);
        self.ready = connected && landed && self.h2c.send(mmio, w0, w1);
        let aid = mmio.read32(REG_BCN_PSR_RPT) & !AID_MASK;
        mmio.write32(REG_BCN_PSR_RPT, aid | u32::from(bss.aid) & AID_MASK);
        mmio.write8(REG_DTIM_COUNTER_ROOT, bss.dtim.max(1) - 1);
    }

    /// Awake, and the station reported gone where it was reported connected.
    pub fn left<M: Mmio>(&mut self, mmio: &M) {
        self.wake(mmio);
        if self.ready {
            let (w0, w1) = media_status(false, 0);
            let _ = self.h2c.send(mmio, w0, w1);
        }
        self.ready = false;
    }
}
