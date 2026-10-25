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
use super::{address, engines, filters, link, phy, port, reset, rings};

pub fn bring_up(driver: &mut Driver) -> Result<(), &'static str> {
    let mut stage = Stage::Reset;
    while stage != Stage::Up {
        enter(stage, driver)?;
        stage = stage.next();
    }
    Ok(())
}

fn enter(stage: Stage, d: &mut Driver) -> Result<(), &'static str> {
    let (regs, part) = (d.regs, d.part);
    match stage {
        Stage::Reset => reset::run(&regs, part),
        Stage::Port => port::copper(&regs),
        Stage::Address => {
            d.mac = address::draw()?;
            filters::program(&regs, part, &d.mac);
            Ok(())
        }
        Stage::Link => {
            link::run(&regs, part);
            Ok(())
        }
        Stage::Phy => phy::wake::run(&regs, part),
        Stage::Engines => {
            engines::enable(&regs);
            Ok(())
        }
        Stage::TxRing => rings::tx::program(&regs, &d.tx, d.tx_ring_device_addr),
        Stage::RxRing => rings::rx::program(&regs, &d.rx, d.rx_ring_device_addr),
        Stage::Up => Ok(()),
    }
}
