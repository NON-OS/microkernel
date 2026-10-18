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

//! A report through the poll loop's own step into the state of the
//! interface it came from, as the loop hands it over after a read.

use crate::enumerate::HidEndpoint;
use crate::poll::feed_report::feed_report;
use crate::state::State;

pub fn feed(state: &mut State, ep: &mut HidEndpoint, report: &[u8]) {
    let mut buf = [0u8; 64];
    buf[..report.len()].copy_from_slice(report);
    feed_report(state, ep, &buf, report.len());
}
