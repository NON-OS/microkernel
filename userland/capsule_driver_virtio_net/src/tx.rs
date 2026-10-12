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

//! Transmit: queue the frame, ring the doorbell, answer. Nothing waits on
//! the device; buffers it has finished with come back on later sends.

use crate::clock::now_ms;
use crate::constants::Q_TX;
use crate::queue::TxError;
use crate::setup::Driver;

/// Queue `frame`. `Full` while every buffer is still the device's;
/// `Stalled` once the ring has stayed full, with the link up and nothing
/// coming back, for TX_STALL_MS: the caller then resets the device.
pub fn send(driver: &mut Driver, frame: &[u8]) -> Result<(), TxError> {
    match driver.tx.post(frame) {
        Ok(()) => {
            driver.transport.notify(Q_TX);
            Ok(())
        }
        Err(TxError::Full) if driver.tx.stalled(now_ms(), driver.link_up()) => {
            Err(TxError::Stalled)
        }
        Err(e) => Err(e),
    }
}
