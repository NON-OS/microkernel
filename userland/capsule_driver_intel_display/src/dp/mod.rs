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

// The file is not called aux.rs: Windows reserves that name, and a checkout
// there fails on it.
#[path = "aux_channel.rs"]
pub mod aux;
pub mod caps;
pub mod dpcd;
pub mod i2c;
pub mod lane;
pub mod link;
pub mod link_check;
pub mod native;
pub mod reply;
pub mod status;
pub mod train;
pub mod train_cr;
pub mod train_eq;
