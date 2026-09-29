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

use super::seal::seal;
use super::window::device_lba;
use super::CryptoBlockError;

/// Seal one block and hold it for the device, unflushed. A caller writing
/// many blocks flushes once after the last, before anything points at them;
/// consecutive blocks reach the device as one request.
pub fn write_deferred(key: &[u8; 32], lba: u64, plain: &[u8]) -> Result<(), CryptoBlockError> {
    let at = device_lba(lba)?;
    let sector = seal(key, lba, plain)?;
    super::pending::hold(at, &sector)
}
