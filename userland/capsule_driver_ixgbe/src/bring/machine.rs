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

//! Running the stages in order until the part is up; the first that fails
//! ends the attempt with its own error line.

use crate::setup::Driver;

use super::stage::Stage;
use super::{address, filters, mode, own, receiver, reset, rings, settings};

pub fn bring_up(driver: &mut Driver) -> Result<(), &'static str> {
    // AUTOC as the reset found it, for the Mode stage.
    let mut autoc = 0;
    let mut stage = Stage::Reset;
    while stage != Stage::Up {
        enter(stage, driver, &mut autoc)?;
        stage = stage.next();
    }
    Ok(())
}

fn enter(stage: Stage, d: &mut Driver, autoc: &mut u32) -> Result<(), &'static str> {
    let (regs, part) = (d.regs, d.part);
    match stage {
        Stage::Reset => *autoc = reset::run(&regs)?,
        Stage::Mode => mode::restore(&regs, part, *autoc),
        Stage::Own => own::take(&regs),
        Stage::Address => {
            d.mac = address::draw()?;
            filters::program(&regs, &d.mac);
        }
        Stage::Settings => settings::program(&regs),
        Stage::TxRing => rings::tx::program(&regs, &d.tx, d.tx_ring_device_addr)?,
        Stage::RxRing => rings::rx::program(&regs, &d.rx, d.rx_ring_device_addr)?,
        Stage::Receiver => receiver::enable(&regs, part),
        Stage::Up => {}
    }
    Ok(())
}
