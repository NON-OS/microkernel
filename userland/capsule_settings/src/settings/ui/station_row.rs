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
//! The Network page's address row: where the wired card's address came from
//! (wifi/station.rs). A list of one entry when the driver said, of none when
//! it did not, so the page never shows a label with nothing beside it; with
//! the reason on the line under the label when the value needs one.

use nonos_app_skeleton::PaintBuffer;

use super::control_geom::right_edge;
use super::metrics::{BODY_PX, ROW_H, ROW_NOTE_H};
use super::theme::SUBLABEL_FG;
use super::walk::Item;
use super::walk_list::List;
use super::{row_label, text};
use crate::settings::state::State;
use crate::wifi::station::{rows, said};

const LABEL: &str = "Network address";

/// The row as a list: its entries, their item and their height.
pub fn station_list(state: &State) -> List {
    let (n, noted) = rows(state.wifi_diag.station);
    (n, |_| Item::Station, if noted { ROW_NOTE_H } else { ROW_H })
}

pub fn paint(fb: &mut PaintBuffer, state: &State, x: u32, w: u32, y: i32, h: u32) {
    let Some((value, note)) = state.wifi_diag.station.and_then(said) else { return };
    row_label::paint(fb, x, y, h, LABEL, note);
    let top = text::centred_top(0, h, BODY_PX) + y;
    text::right(fb, right_edge(x, w), top, value, SUBLABEL_FG, BODY_PX);
}
