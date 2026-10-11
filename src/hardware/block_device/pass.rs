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

//! What one selection pass has learnt of the block wire's endpoints. Each
//! unit of an endpoint is its own row, and a request to a capsule that
//! does not answer costs the whole transport timeout; once one unit of an
//! endpoint fails for a transport reason, its other units are not asked
//! this pass, so a hung UFS host costs one timeout, not eight.
//!
//! Pure, so kernel_proofs holds it.

use super::BlockDeviceError;

#[derive(Clone, Copy, Default)]
pub(super) struct Pass {
    /// Bit n: endpoint n stopped answering in this pass.
    silent: u64,
}

impl Pass {
    /// Whether a row on `endpoint` is not worth asking; rows off the block
    /// wire carry no endpoint and are always asked.
    pub(super) fn skips(&self, endpoint: Option<u8>) -> bool {
        endpoint.is_some_and(|n| n < 64 && self.silent & 1 << n != 0)
    }

    pub(super) fn refused(&mut self, endpoint: Option<u8>, e: BlockDeviceError) {
        if let (Some(n), BlockDeviceError::TransportFailure) = (endpoint, e) {
            if n < 64 {
                self.silent |= 1 << n;
            }
        }
    }
}
