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

pub mod accounts;
pub mod amounts;
pub mod backup;
pub mod backup_words;
pub mod custody;
pub mod hits;
pub mod home;
mod home_actions;
mod home_columns;
mod home_pills;
mod home_tiles;
pub mod locked;
pub mod meter;
pub mod page;
pub mod pay;
pub mod publisher;
pub mod purchase;
pub mod receive;
pub mod receive_address;
mod receive_body;
mod receive_words;
pub mod settings;
pub mod shield;
pub mod stake;
pub mod status;
mod status_words;
pub mod swap;
pub mod welcome;
