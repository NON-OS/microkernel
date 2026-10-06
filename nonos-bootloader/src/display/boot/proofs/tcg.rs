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

//! The firmware's TCG log, replayed here only to count its events and find
//! the last boot application the firmware measured. Whether the replay meets
//! the live PCR 4 is the kernel's check, not this one.

use nonos_boot_measure::tcg::replay;

use super::rows::Row;
use super::state::State;
use crate::display::text::Text;

pub fn tcg_log(log: Option<&[u8]>) -> Row {
    let t = Text::new();
    let (state, detail) = match log.filter(|b| !b.is_empty()).map(replay) {
        None => (State::Absent, t.push(b"no log from the firmware")),
        Some(Err(e)) => (State::Failed, t.push(b"does not replay, code ").dec(e.code() as u64)),
        Some(Ok(r)) if r.last_application.is_some() => (
            State::Present,
            t.dec(r.events as u64).push(b" events, boot app logged, kernel checks PCR 4"),
        ),
        Some(Ok(r)) => {
            (State::NotMeasured, t.dec(r.events as u64).push(b" events, no boot application event"))
        }
    };
    Row { label: b"TCG LOG", state, detail }
}
