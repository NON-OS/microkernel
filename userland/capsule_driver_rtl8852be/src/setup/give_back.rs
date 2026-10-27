// NONOS Operating System (AGPL-3.0-or-later)

//! A chip that stopped bring-up after its claim: the reason on the console,
//! the MAC powered off, Bus Master dropped and the claim given back (which
//! takes the register window and every DMA region with it). The station
//! then serves the stage alone.

use super::master::release;
use crate::pwr;
use crate::regs::Regs;
use crate::stage::Stage;
use crate::status;

pub fn give_back<T>(r: &Regs, id: u64, epoch: u64, stage: Stage, why: &[u8]) -> (Option<T>, Stage) {
    status::line(why);
    let _ = pwr::switch(r, false, &pwr::Board::default());
    release(id, epoch);
    (None, stage)
}
