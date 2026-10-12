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

//! A hostile device, played by a seeded generator: configuration
//! descriptors and report streams, two hundred thousand rounds of each,
//! through the driver's own decode. Nothing may panic, and nothing may come
//! out that lies outside the defined key, button and position ranges.

mod config_tests;
mod hostile;
mod key_report;
mod key_stream_tests;
pub mod mouse_stream_tests;
mod reference;
mod rounds;

pub use rounds::ROUNDS;
