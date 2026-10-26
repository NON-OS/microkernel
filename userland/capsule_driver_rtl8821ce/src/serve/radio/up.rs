// NONOS Operating System (AGPL-3.0-or-later)

//! The radio held up or down, and the station address in the MAC-ID registers.

use alloc::boxed::Box;

use super::super::Stage;
use super::phy::phy_setup;
use crate::constants::regs::REG_MACID;
use crate::fw::dma::Grant;
use crate::link::{RtlKeys, RtlLink};
use crate::ps::Ps;
use crate::regs::{Mmio, Regs};
use crate::setup::Mapped;
use crate::status;

/// The radio brought up: its rings, its key store and a register handle for
/// retuning during a scan. Boxed in `Radio` so the down case stays small.
pub(in super::super) struct RadioUp {
    pub(in super::super) link: RtlLink<Regs, Grant, Grant>,
    pub(in super::super) keys: RtlKeys<Regs>,
    pub(in super::super) regs: Regs,
}

/// The radio as the serve loop holds it: either fully up, or down, in which case
/// the loop still answers so the service stays registered and the panel can read
/// the failure stage.
pub(in super::super) enum Radio {
    Up(Box<RadioUp>),
    Down,
}

/// Bring the radio to `Up` when bring-up reached `Ready` with its rings mapped
/// (`start_ready` maps them before the engines run), else leave it `Down`.
/// Returns the possibly-downgraded stage, so a failure here is reported too.
pub(in super::super) fn build_radio(mapped: Option<Mapped>, stage: Stage) -> (Radio, Stage) {
    if stage != Stage::Ready {
        return (Radio::Down, stage);
    }
    let Some(mut m) = mapped else {
        return (Radio::Down, Stage::NotClaimed);
    };
    let Some((tx_ring, tx_buffers, rx_ring, rx_buffers)) = m.rings.take() else {
        status::line(b"[rtl8821ce] ready without its rings\n");
        crate::setup::stopped(&m);
        return (Radio::Down, Stage::NoDma);
    };
    if let Err(stage) = phy_setup(&m.regs, m.efuse) {
        crate::setup::stopped(&m);
        return (Radio::Down, stage);
    }
    let our_mac = read_mac(&m.regs);
    let ps = Ps::new(crate::setup::host_aspm(&m));
    let mut link = RtlLink::new(m.regs, tx_ring, tx_buffers, rx_ring, rx_buffers, our_mac);
    link.set_ps(ps, m.beacon.take());
    link.set_rfe(m.efuse.map_or(0, |e| e.rfe));
    let keys = RtlKeys::new(m.regs);
    (Radio::Up(Box::new(RadioUp { link, keys, regs: m.regs })), Stage::Ready)
}

/// The station MAC from the MAC-ID registers, as `phy_setup` programmed it.
fn read_mac(regs: &Regs) -> [u8; 6] {
    let lo = regs.read32(REG_MACID);
    let hi = regs.read32(REG_MACID + 4);
    [lo as u8, (lo >> 8) as u8, (lo >> 16) as u8, (lo >> 24) as u8, hi as u8, (hi >> 8) as u8]
}
