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

//! The panel's lines in the boot log: the console log, which the firmware
//! may mirror to serial, and the splash's log panel.

use super::rows::Row;
use super::state::State;
use crate::display::log_panel;
use crate::display::text::Text;
use crate::log::logger;

const CATEGORY: &str = "proofs";

pub fn log_row(row: &Row) {
    let line = Text::new().push(row.label).push(b" ").push(row.state.word());
    let line = line.push(b": ").push(row.detail.as_bytes());
    let bytes = line.as_bytes();
    let text = core::str::from_utf8(bytes).unwrap_or("proofs line not ASCII");
    match row.state {
        State::Failed => {
            logger::log_error(CATEGORY, text);
            log_panel::log_error(bytes);
        }
        State::Absent | State::NotMeasured | State::Off => {
            logger::log_warn(CATEGORY, text);
            log_panel::log_warn(bytes);
        }
        State::Verified | State::Present | State::On => {
            logger::log_info(CATEGORY, text);
            log_panel::log_ok(bytes);
        }
    }
}
