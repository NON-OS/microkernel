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
//! A part that finishes each MAC reset on the next look: CTRL.RST drops,
//! the reset reloads AUTOC from the EEPROM (`eeprom_autoc`), and EEC reports
//! the auto-read done. The security block reports ready.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use nonos_devmodel::FakeBar;

use crate::facts::mac::*;
use crate::facts::regs::*;

use super::model::stable32;

pub fn part(eeprom_autoc: u32, resets: Arc<AtomicU32>) -> impl Fn(&FakeBar) + Send {
    move |b| {
        let ctrl = stable32(b, REG_CTRL).unwrap_or(0);
        if ctrl & CTRL_RST != 0 {
            b.present32(REG_CTRL, ctrl & !(CTRL_RST | CTRL_GIO_DIS));
            b.present32(REG_AUTOC, eeprom_autoc);
            b.present32(REG_EEC, EEC_ARD);
            resets.fetch_add(1, Ordering::SeqCst);
        }
        b.present32(REG_SECRXSTAT, SECRXSTAT_RDY);
    }
}
