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

//! The engine's copy of a transfer's bytes goes with its result. The
//! receive FIFO is drained into it and a reply is built from it, so it
//! holds what the device returned, an input report among it; it is wiped
//! wherever it is let go: once the reply is sent, on an error, and in
//! setup's probes.

use super::transfer_result::TransferResult;
use crate::protocol::wipe;

impl Drop for TransferResult {
    fn drop(&mut self) {
        wipe(&mut self.read);
    }
}
