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

//! The console lines: the driver up with its codec, and per stream the
//! first period played, the first underrun and the totals at stop. Every
//! other period is only counted, so a long playback cannot fill the log.

mod line;

use crate::audio::Periods;
use crate::codec::Ident;
use line::Line;

pub fn up(id: &Ident) {
    let mut line = Line::new(b"driver.ac97: up, codec ");
    line.hex16(id.vendor).push(b":").hex16(id.device).push(b", 48 kHz stereo").say();
}

pub fn first_period() {
    Line::new(b"driver.ac97: playing").say();
}

pub fn first_underrun() {
    Line::new(b"driver.ac97: underrun: the queue ran dry; counted, totals at stream stop").say();
}

pub fn totals(p: &Periods) {
    if p.any() {
        let mut line = Line::new(b"driver.ac97: stream stop: ");
        line.dec(p.played).push(b" periods played, ").dec(p.underruns).push(b" underruns").say();
    }
}
