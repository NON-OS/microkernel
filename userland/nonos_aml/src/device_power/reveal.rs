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

//! Devices firmware left so far off that enumeration saw nothing: their D0
//! runs and the kernel probes the one function. One that never answers,
//! or that no driver claims by the end of boot, goes back to D3cold.

use super::pool::PowerPool;
use super::rules::MoveError;
use super::state::DState;
use super::transition::DevicePower;
use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::Eval;
use alloc::vec::Vec;

#[derive(Debug)]
pub enum Revealed {
    /// The function answered after D0 and stays on, waiting for a driver.
    Answered,
    /// It never answered, and is back in D3cold.
    Silent,
    /// D0 itself failed; nothing was left on.
    Failed(MoveError),
}

pub fn reveal(
    dp: &mut DevicePower,
    pool: &mut PowerPool,
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    dev: Id,
    probe: &mut dyn FnMut() -> bool,
) -> Revealed {
    if let Err(e) = dp.set(pool, aml, eval, dev, DState::D3Cold, DState::D0) {
        return Revealed::Failed(e);
    }
    if probe() {
        return Revealed::Answered;
    }
    let _ = dp.set(pool, aml, eval, dev, DState::D0, DState::D3Cold);
    Revealed::Silent
}

/// End of boot: each answered device no driver claimed goes to D3cold.
pub fn settle(
    dp: &mut DevicePower,
    pool: &mut PowerPool,
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    answered: &[Id],
    claimed: &mut dyn FnMut(Id) -> bool,
) -> Vec<Id> {
    let mut down = Vec::new();
    for &dev in answered {
        let on = dp.state(dev) == Some(DState::D0);
        if on && !claimed(dev) && dp.set(pool, aml, eval, dev, DState::D0, DState::D3Cold).is_ok() {
            down.push(dev);
        }
    }
    down
}
