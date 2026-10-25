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

//! `neofetch`: the system's facts beside the emblem, in the fastfetch style.

use crate::command::output::Output;
use crate::term::state::State;
use crate::term::theme::profiles::by_index;

use super::compose::two_column;
use super::emblem::emblem;
use super::palette::palette;

#[path = "facts.rs"]
mod facts;
#[path = "gather.rs"]
mod gather;
#[path = "run_info.rs"]
mod run_info;

use gather::read;
use run_info::{info, Facts};

const GAP: usize = 2;

pub fn run(state: &mut State) {
    let r = read(state);
    let f = Facts {
        kernel: &r.kernel,
        uptime: &r.uptime,
        signed: crate::command::builtin::receipt::own_line(),
        memory: &r.memory,
        processes: &r.processes,
        load: &r.load,
        terminal: &r.terminal,
        theme: r.theme,
    };
    let bg = by_index(state.scrollback.theme_of.unwrap_or(0)).bg & 0x00FF_FFFF;
    let rows = two_column(&emblem(bg), &info(&f), GAP);
    let out = &mut Output::new(&mut state.scrollback);
    for (plain, styled) in &rows {
        out.writeln_styled(plain, styled);
    }
    out.writeln(b"");
    for strip in 0..2 {
        let (plain, styled) = palette(strip);
        out.writeln_styled(&plain, &styled);
    }
}
