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

//! The wallet's screens on the Etna frame, one file each, with the shared
//! status line, balances and the table of what can be pressed.

pub mod amounts;
pub mod backup;
pub mod backup_words;
pub mod hits;
pub mod home;
mod home_actions;
mod home_pills;
pub mod receive;
pub mod receive_address;
pub mod status;
pub mod welcome;
