// NONOS Operating System (AGPL-3.0-or-later)

//! Start: find and claim the chip, power a MAC a warm reboot left on off
//! before Bus Master goes on (so no DMA a dead driver armed runs), map the
//! rings, probe (`rtw89_chip_info_setup`), start (`rtw89_core_start`) and
//! add the station interface (`rtw89_ops_add_interface`). A step that
//! fails names itself in the stage byte, and the chip is powered off,
//! Bus Master dropped and the claim given back: never left armed while the
//! station serves "down".

use crate::bring::{probe::probe, start::start};
use crate::dma::Grant;
use crate::h2c::Fw;
use crate::regs::Regs;
use crate::ring::map_rings;
use crate::setup::give_back;
use crate::sta::chip::{Chip, Vif};
use crate::stage::{stage_of, Stage};
use crate::{discover, pwr, setup, status};
use nonos_libc::{mk_exit, start_driver, EXIT_ABSENT};

const DRIVER: &[u8] = b"driver.rtl8852be";

pub type Up = Chip<Regs, Grant>;

pub fn bring_up() -> (Option<Up>, Stage) {
    let c = match start_driver(DRIVER, discover::find(), |dev| setup::run(*dev)) {
        Ok(c) => c,
        Err(EXIT_ABSENT) => mk_exit(EXIT_ABSENT),
        Err(_) => {
            let (step, errno) = setup::claim_note::read();
            let mut l = status::Line::new(b"[rtl8852be] claim or map refused, step ");
            l.dec(step).text(b" errno ").dec(errno).emit();
            return (None, Stage::NotClaimed);
        }
    };
    let (r, id, epoch) = (c.regs, c.device_id, c.epoch);
    let _ = pwr::switch(&r, false, &pwr::Board::default());
    if !setup::command(id, epoch, true) {
        return give_back(&r, id, epoch, Stage::NotClaimed, b"[rtl8852be] bus master refused\n");
    }
    let Some(mut rings) = map_rings(id, epoch) else {
        return give_back(&r, id, epoch, Stage::NoDma, b"[rtl8852be] ring mapping failed\n");
    };
    let mut fw = Fw::default();
    let p = match probe(&r, &mut rings, &mut fw) {
        Ok(p) => p,
        Err(e) => return give_back(&r, id, epoch, stage_of(e), b"[rtl8852be] probe stopped\n"),
    };
    status::line(b"[rtl8852be] firmware up, efuse and phy capabilities read\n");
    if p.caps.rx_b_only {
        let why = b"[rtl8852be] one-antenna board not supported\n";
        return give_back(&r, id, epoch, Stage::NoAirPath, why);
    }
    let s = match start(&r, &mut rings, &mut fw, &p) {
        Ok(s) => s,
        Err(e) => return give_back(&r, id, epoch, stage_of(e), b"[rtl8852be] start stopped\n"),
    };
    if !s.rf_pages_sent {
        status::line(b"[rtl8852be] firmware did not take every rf register page\n");
    }
    let Some(sma) = crate::station::draw() else {
        let why = b"[rtl8852be] no entropy for an address\n";
        return give_back(&r, id, epoch, Stage::NoStationAddress, why);
    };
    let (phy, rfk, vif) = (s.phy, s.rfk, Vif::new(sma));
    let mut chip = Chip { r, rings, fw, phy, rfk, chan: None, vif };
    if chip.vif_init().is_err() {
        let why = b"[rtl8852be] station interface set-up failed\n";
        return give_back(&chip.r, id, epoch, Stage::ChipInit, why);
    }
    status::line(b"[rtl8852be] station interface ready\n");
    (Some(chip), Stage::Ready)
}
