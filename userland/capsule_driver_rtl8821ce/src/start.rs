// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Bringing the radio up at start, as far as it goes.

use nonos_libc::{mk_exit, start_driver, EXIT_ABSENT, EXIT_GAVE_UP};

use crate::bringup::probe;
use crate::pwr::NoMaster;
use crate::serve::Stage;
use crate::setup::Mapped;
use crate::start_board::link_and_board;
use crate::start_ready::up_to_ready;
use crate::{discover, setup, status};

const DRIVER: &[u8] = b"driver.rtl8821ce";

/// Claim the device, power the MAC on, load the firmware and initialise the MAC,
/// stopping at the first step that fails and reporting how far it got. On a
/// serial-less machine the returned stage is the only way to see this, surfaced
/// through the panel's status request.
///
/// A machine without the chip is the one case that does not serve: the driver
/// says so and leaves (`EXIT_ABSENT`) before claiming anything, since there is
/// no stage to report. The claim and register map are retried on the shared
/// bounded schedule, each failed try giving the claim back; running out serves
/// `NotClaimed`, so the panel can still say why. A chip whose power cycle or
/// registers fail, or that is refused Bus Master, is given back and the
/// driver leaves (`EXIT_GAVE_UP`): it is never served in an unknown state.
pub fn bring_up() -> (Option<Mapped>, Stage) {
    let mut mapped = match start_driver(DRIVER, discover::find(), |dev| setup::run(*dev)) {
        Ok(m) => m,
        Err(EXIT_ABSENT) => mk_exit(EXIT_ABSENT),
        Err(_) => {
            status::line(b"[rtl8821ce] could not claim or map the chip\n");
            return (None, Stage::NotClaimed);
        }
    };
    let (id, epoch) = (mapped.device_id, mapped.claim_epoch);
    if let Err(why) = probe(&mapped.regs, || setup::command(id, epoch, true)) {
        status::line(refusal(why));
        setup::release(&mapped);
        mk_exit(EXIT_GAVE_UP);
    }
    status::line(b"[rtl8821ce] chip powered and answering; bus master on\n");
    link_and_board(&mut mapped);
    up_to_ready(mapped)
}

/// The console line for a chip that is given back.
fn refusal(why: NoMaster) -> &'static [u8] {
    match why {
        NoMaster::PowerFailed => b"[rtl8821ce] power-on sequence failed; chip given back\n",
        NoMaster::LeftOn { .. } => b"[rtl8821ce] power-off did not take; chip given back\n",
        NoMaster::DeadMmio => b"[rtl8821ce] no response after power-on; chip given back\n",
        NoMaster::Refused => b"[rtl8821ce] bus master refused; chip given back\n",
    }
}
