// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::map_block::map_block_error;
use super::write_deferred::write_deferred;
use super::CryptoBlockError;

pub fn write(key: &[u8; 32], lba: u64, plain: &[u8]) -> Result<(), CryptoBlockError> {
    write_deferred(key, lba, plain)?;
    super::pending::drain()?;
    super::device::flush().map_err(map_block_error)
}
