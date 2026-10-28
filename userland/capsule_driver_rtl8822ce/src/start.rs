// NONOS Operating System (AGPL-3.0-or-later)

//! Bringing the radio up at start, as far as it goes: the claim and map
//! (`setup::run`, the RTL8821CE's, on the shared bounded schedule), power-on
//! with Bus Master only after it (`bringup::probe`), the link and the board
//! (`start_board`), then the firmware and the MAC (`start_ready`). A chip
//! that is not there leaves at once (`EXIT_ABSENT`); one that will not power
//! on is given back (`EXIT_GAVE_UP`); any later stop is served as its stage.

use nonos_libc::{mk_exit, start_driver, EXIT_ABSENT, EXIT_GAVE_UP};

use crate::bringup::probe;
use crate::serve::Stage;
use crate::setup::Mapped;
use crate::start_board::link_and_board;
use crate::start_ready::up_to_ready;
use crate::start_say::{own, refusal};
use crate::{discover, setup, status};

const DRIVER: &[u8] = b"driver.rtl8822ce";

/// Claim, power, and load the chip, stopping at the first step that fails.
pub fn bring_up() -> (Option<Mapped>, Stage) {
    let claim = |dev: &discover::Found| setup::run(*dev).map_err(own);
    let mut mapped = match start_driver(DRIVER, discover::find(), claim) {
        Ok(m) => m,
        Err(EXIT_ABSENT) => mk_exit(EXIT_ABSENT),
        Err(_) => {
            status::line(b"[rtl8822ce] could not claim or map the chip\n");
            return (None, Stage::NotClaimed);
        }
    };
    let (id, epoch) = (mapped.device_id, mapped.claim_epoch);
    if let Err(why) = probe(&mapped.regs, || setup::command(id, epoch, true)) {
        status::line(refusal(why));
        setup::release(&mapped);
        mk_exit(EXIT_GAVE_UP);
    }
    status::line(b"[rtl8822ce] chip powered and answering; bus master on\n");
    link_and_board(&mut mapped);
    up_to_ready(mapped)
}
