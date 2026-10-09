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

//! Wi-Fi and Bluetooth share the 8821CE's antenna. rtw88 hands it to Wi-Fi
//! in its coexistence setup (coex.c, rtw8821c.c) after every power-on; this
//! driver runs no Bluetooth, so it takes the Wi-Fi-only path.

mod indirect;
mod init;
pub mod switch;
mod wl_only;

pub use wl_only::take_antenna;
