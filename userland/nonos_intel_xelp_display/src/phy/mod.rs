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
//! The combo PHY: its one-time initialisation, the voltage swing on its
//! lanes, its IO power, and which lanes it powers (Vol 12 pp. 390 to 395).

pub mod init;
pub mod lanes;
pub mod power;
pub mod procmon;
pub mod signal;
pub mod swing;
pub mod swing_hdmi;
pub mod swing_table;
