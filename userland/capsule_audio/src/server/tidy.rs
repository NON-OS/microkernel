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

//! What the server looks at every two seconds: the streams of clients that
//! ended, and whether a USB device came ready or went.

use nonos_libc::mk_pid_alive;

use super::pump::PumpState;
use super::streams::StreamTable;
use crate::sink::Sink;

fn alive(pid: u32) -> bool {
    mk_pid_alive(pid)
}

pub(super) fn tidy(table: &mut StreamTable, pump: &mut PumpState, sink: Option<&Sink>) {
    table.reap(alive);
    if sink.is_some_and(Sink::rechoose) {
        pump.moved();
    }
}
