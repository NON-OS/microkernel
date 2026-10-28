// NONOS Operating System (AGPL-3.0-or-later)

//! Everything after the claim, over any register window and DMA memory, so
//! the host proofs run the very order the capsule runs: probe
//! (`rtw89_chip_info_setup`), start (`rtw89_core_start`), the station
//! address, then the station interface (`rtw89_ops_add_interface`). A step
//! that fails hands the window back with the stage it reached and the line
//! that names it, for the caller to power off and release.

use super::{probe::probe, start::start};
use crate::dma::DmaMem;
use crate::h2c::Fw;
use crate::regs::Mmio;
use crate::ring::Rings;
use crate::sta::chip::{Chip, Vif};
use crate::stage::{stage_of, Stage};
use crate::status;

/// The window back, the stage reached, and the line to log.
pub type Failed<R> = (R, Stage, &'static [u8]);

pub fn up<R: Mmio, M: DmaMem>(
    r: R,
    mut rings: Rings<M>,
    draw: impl FnOnce() -> Option<[u8; 6]>,
) -> Result<Chip<R, M>, Failed<R>> {
    let mut fw = Fw::default();
    let p = match probe(&r, &mut rings, &mut fw) {
        Ok(p) => p,
        Err(e) => return Err((r, stage_of(e), b"[rtl8852be] probe stopped\n")),
    };
    status::line(b"[rtl8852be] firmware up, efuse and phy capabilities read\n");
    if p.caps.rx_b_only {
        return Err((
            r,
            Stage::NoAirPath,
            b"[rtl8852be] one-antenna board: only two-antenna boards are driven\n",
        ));
    }
    let s = match start(&r, &mut rings, &mut fw, &p) {
        Ok(s) => s,
        Err(e) => return Err((r, stage_of(e), b"[rtl8852be] start stopped\n")),
    };
    if !s.rf_pages_sent {
        status::line(b"[rtl8852be] firmware did not take every rf register page\n");
    }
    let Some(sma) = draw() else {
        return Err((r, Stage::NoStationAddress, b"[rtl8852be] no entropy for an address\n"));
    };
    let (phy, rfk, vif) = (s.phy, s.rfk, Vif::new(sma));
    let mut chip = Chip { r, rings, fw, phy, rfk, chan: None, vif };
    if chip.vif_init().is_err() {
        return Err((chip.r, Stage::ChipInit, b"[rtl8852be] station interface set-up failed\n"));
    }
    // TSSI and DPK are not ported (`rfk`): the line says so.
    status::line(b"[rtl8852be] station interface ready; tx power open loop, no TSSI or DPK\n");
    Ok(chip)
}
