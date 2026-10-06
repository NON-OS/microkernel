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

use alloc::vec::Vec;

use super::bring_up::bring_up;
use crate::discover::choice::{choose, settled, Choice, Outcome};
use crate::discover::Candidates;
use crate::error::{NvmeError, NvmeResult};
use crate::setup::{hmb, say, Driver};

/// One bring-up attempt over the controllers discovery found, in the order
/// it ranked them. Each is brought up in turn until a disk with an I/O queue
/// is up; the one `choice::choose` picks is served and every other is
/// dropped, which disables it and releases what it held. An attempt that
/// serves nothing fails, and the shared schedule tries again.
pub fn run(found: &Candidates) -> NvmeResult<Driver> {
    let mut up: Vec<Option<Driver>> = Vec::new();
    let mut tries: Vec<(bool, Outcome)> = Vec::new();
    let mut last_err = NvmeError::UnsupportedController;
    for dev in found.iter() {
        let asked = hmb::extras();
        let outcome = match bring_up(*dev) {
            Ok(driver) => {
                let outcome = if driver.io.is_some() { Outcome::Io } else { Outcome::NoIo };
                up.push(Some(driver));
                outcome
            }
            Err(e) => {
                say::attempt_failed(dev, e);
                if asked {
                    hmb::extras_failed();
                }
                last_err = e;
                up.push(None);
                Outcome::Failed
            }
        };
        tries.push((dev.cache, outcome));
        if settled(dev.cache, outcome) {
            break;
        }
    }
    match choose(&tries) {
        Choice::Serve(i) => match up.get_mut(i).and_then(Option::take) {
            Some(driver) => {
                say::serving(&driver);
                Ok(driver)
            }
            None => Err(last_err),
        },
        Choice::Retry => Err(last_err),
    }
}
