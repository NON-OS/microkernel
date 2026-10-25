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

//! The driver's loop. A device not yet ready is brought up again every half
//! second; one that is, is read when its interrupt line rang. Until the
//! line has rung once it is not trusted, and reads go by timer as they do
//! where the firmware named no line or gpio0 cannot say.

use nonos_libc::mk_idle_ms;

use crate::bring_up::bring_up;
use crate::input::poll;
use crate::link::Link;
use crate::spi::doorbell;
use crate::state::State;

const RETRY_MS: u64 = 500;
/// A quiet, trusted line is looked at again this soon.
const LINE_MS: u64 = 2;
/// Reads by timer: a pad reports about every 8 ms.
const TIMER_MS: u64 = 4;

pub fn run(l: Link, mut state: State) -> ! {
    loop {
        if !state.found() {
            if let Err(why) = bring_up(&l, &mut state) {
                say_once(&mut state, why);
                let _ = mk_idle_ms(RETRY_MS);
                continue;
            }
            crate::diag::line(alloc::string::String::from("driver.spi_hid: device ready\n"));
        }
        let rang = l.choice.line.and_then(|(uid, pin)| doorbell(l.port, uid, pin));
        state.doorbell_proven |= rang == Some(true);
        match rang {
            Some(true) => poll(&mut state),
            Some(false) if state.doorbell_proven => {
                let _ = mk_idle_ms(LINE_MS);
            }
            _ => {
                poll(&mut state);
                let _ = mk_idle_ms(TIMER_MS);
            }
        }
    }
}

fn say_once(state: &mut State, why: &str) {
    if !state.said {
        state.said = true;
        crate::diag::line(alloc::format!("{why}\n"));
    }
}
