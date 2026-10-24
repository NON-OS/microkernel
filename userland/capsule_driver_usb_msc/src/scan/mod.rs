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

//! Finding the one mass-storage device this driver serves, and keeping it,
//! one bounded step at a time between requests.

mod ask;
mod asked;
mod asking;
mod bot_bind;
mod check;
mod choose;
mod command;
mod configure;
mod controller;
mod identity;
mod keep;
mod look;
mod medium;
mod pace;
mod ports;
mod probe;
mod ready_size;
mod release;
pub mod report;
mod scanner;
mod search;
mod searching;
mod sized;
mod step;
mod uas_bind;
mod uas_setup;
mod units;

pub use medium::Medium;
pub use scanner::Scanner;
