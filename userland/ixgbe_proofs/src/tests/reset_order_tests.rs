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
//! The one ordering the master stop exists for: CTRL.RST is never written
//! while STATUS still shows a master request outstanding. A modelled part
//! stops only once it sees GIO_DIS and watches for a reset before then.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::bring::reset::run;
use crate::facts::mac::*;
use crate::facts::regs::*;

use super::model::{live, regs, stable32, window};

#[test]
fn the_reset_never_lands_while_a_master_request_is_out() {
    let bar = window();
    bar.present32(REG_STATUS, STATUS_GIO);
    let early = Arc::new(AtomicBool::new(false));
    let seen = early.clone();
    let _part = live(&bar, move |b| {
        let Some(ctrl) = stable32(b, REG_CTRL) else { return };
        let mastering = b.wrote32(REG_STATUS) & STATUS_GIO != 0;
        if ctrl & CTRL_RST != 0 && mastering {
            seen.store(true, Ordering::SeqCst);
        }
        if ctrl & CTRL_GIO_DIS != 0 {
            b.present32(REG_STATUS, 0);
        }
        if ctrl & CTRL_RST != 0 {
            b.present32(REG_CTRL, ctrl & !CTRL_RST);
            b.present32(REG_EEC, EEC_ARD);
        }
    });
    assert!(run(&regs(&bar)).is_ok());
    assert!(!early.load(Ordering::SeqCst));
}
