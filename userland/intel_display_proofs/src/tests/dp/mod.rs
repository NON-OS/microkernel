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

//! The DisplayPort source's own proofs: AUX, capabilities, I2C over AUX,
//! lanes, link training and its checks, and a hostile sink against all
//! of them.

mod dp_aux_tests;
mod dp_caps_tests;
mod dp_hostile;
mod dp_i2c_tests;
mod dp_lane_tests;
mod dp_link_check_tests;
mod dp_train_fail_tests;
mod dp_train_tests;
pub mod edid_read;
