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
pub mod home_geom;
mod home_tile;
pub mod lib_geom;
mod library;
mod nowplaying;
mod downloads;
pub mod downloads_geom;
mod search;
pub mod search_geom;
pub mod settings;
mod settings_view;
mod track_cols;
mod track_row;

pub use downloads::{any_finished, paint as downloads};
pub use downloads_geom::visible as downloads_visible;
pub use home::paint as home;
pub use lib_geom::visible as lib_visible;
pub use library::{paint as library, rows_for};
pub use nowplaying::paint as nowplaying;
pub use search::paint as search;
pub use search_geom::visible as search_visible;
pub use settings_view::paint as settings;
