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

mod home;
mod lib_geom;
mod library;
mod nowplaying;
mod downloads;
mod search;
mod settings;
mod settings_view;

pub use home::{card_at, paint as home, see_all_at};
pub use lib_geom::{row_at as lib_row_at, rows_for, tab_hit, visible as lib_visible};
pub use library::paint as library;
pub use nowplaying::paint as nowplaying;
pub use downloads::{
    act_at as download_act_at, any_finished, clear_rect as downloads_clear_rect,
    paint as downloads, visible as downloads_visible,
};
pub use search::{
    clear_rect as search_clear_rect, paint as search, play_at as search_play_at,
    play_rect as search_play_rect, row_at as search_row_at, visible as search_visible,
};
pub use settings::{hit as settings_hit, Hit as SettingsHit};
pub use settings_view::paint as settings;
