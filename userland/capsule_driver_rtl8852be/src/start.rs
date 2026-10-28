// NONOS Operating System (AGPL-3.0-or-later)

//! Start: find and claim the chip, power a MAC a warm reboot left on off
//! before Bus Master goes on (so no DMA a dead driver armed runs), map the
//! rings, then the bring-up proper (`bring::up`). A step that fails names
//! itself in the stage byte, and the chip is powered off, Bus Master
//! dropped and the claim given back: never left armed while the station
//! serves "down".

use crate::bring::up::up;
use crate::dma::Grant;
use crate::regs::Regs;
use crate::ring::map_rings;
use crate::setup::give_back;
use crate::sta::chip::Chip;
use crate::stage::Stage;
use crate::{discover, pwr, setup, station, status};
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
    let Some(rings) = map_rings(id, epoch) else {
        return give_back(&r, id, epoch, Stage::NoDma, b"[rtl8852be] ring mapping failed\n");
    };
    match up(r, rings, station::draw) {
        Ok(chip) => (Some(chip), Stage::Ready),
        Err((r, stage, why)) => give_back(&r, id, epoch, stage, why),
    }
}
