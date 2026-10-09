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

//! The one ordering igc_reset_hw_base exists for: CTRL.RST is never written
//! while STATUS still shows bus mastering. A modelled part stops mastering
//! only once it sees GIO_MASTER_DISABLE and watches for a reset before then.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::constants::ctrl::*;
use crate::constants::regs::*;
use crate::init::reset::run;

use super::model::{live, regs, stable32, window};

#[test]
fn the_reset_never_lands_while_bus_mastering_is_on() {
    let bar = window();
    bar.present32(REG_STATUS, STATUS_GIO_MASTER_ENABLE);
    bar.present32(REG_EECD, EECD_AUTO_RD);
    let early = Arc::new(AtomicBool::new(false));
    let seen = early.clone();
    let _part = live(&bar, move |b| {
        let Some(ctrl) = stable32(b, REG_CTRL) else { return };
        let mastering = b.wrote32(REG_STATUS) & STATUS_GIO_MASTER_ENABLE != 0;
        if ctrl & CTRL_RST != 0 && mastering {
            seen.store(true, Ordering::SeqCst);
        }
        if ctrl & CTRL_GIO_MASTER_DISABLE != 0 {
            b.present32(REG_STATUS, 0);
        }
    });
    assert_eq!(run(&regs(&bar)), Ok(()));
    assert!(!early.load(Ordering::SeqCst));
}
