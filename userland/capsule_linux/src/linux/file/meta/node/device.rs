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

/* The metadata of a character device this capsule answers. */

use super::super::super::dev_stat;
use super::super::statbuf::Meta;
use super::fd::now;
use super::path::at;

/* The device `dev`, by the path it was opened at. */
pub(super) fn device(path: &[u8], dev: u32) -> Option<Meta> {
    let rdev = dev_stat::rdev(dev)?;
    Some(Meta { rdev, ..at(path, dev_stat::MODE, 0, now()) })
}
