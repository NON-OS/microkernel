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

//! struct virtio_input_config: write `select` and `subsel`, then `size`
//! says how many bytes of the union answer (0: nothing there).

pub const SELECT: usize = 0;
pub const SUBSEL: usize = 1;
pub const SIZE: usize = 2;
pub const UNION: usize = 8;
/// The bytes the driver reads: the name, the longest answer it asks for.
pub const CFG_LEN: usize = UNION + NAME_MAX;
pub const CFG_ID_NAME: u8 = 0x01;
pub const CFG_ABS_INFO: u8 = 0x12;
/// The longest name the boot line shows.
pub const NAME_MAX: usize = 32;
/// struct virtio_input_absinfo: min, then max, both le32.
pub const ABS_MIN: usize = UNION;
pub const ABS_MAX: usize = UNION + 4;
