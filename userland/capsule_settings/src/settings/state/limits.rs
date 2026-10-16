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

//! How much the panel holds: a value per policy field, and the most Wi-Fi
//! adapters and networks it lists.

use crate::settings::schema::ALL_FIELDS;

pub const FIELD_SLOTS: usize = ALL_FIELDS.len();
/// The most WiFi adapters the panel lists at once.
pub const WIFI_MAX: usize = 8;
/// The most networks the panel shows from one scan.
pub const WIFI_NET_MAX: usize = 16;
