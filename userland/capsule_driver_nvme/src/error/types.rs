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

#[derive(Debug, Clone, Copy)]
pub enum NvmeError {
    ClaimFailed,
    BrokerCallFailed,
    UnsupportedController,
    UnsupportedPageSize,
    ControllerTimeout,
    /// CSTS.CFS still set once CC.EN is clear and CAP.TO has passed: the
    /// reset that clears it did not.
    ControllerFatal,
    /// The monotonic clock could not be read, so no wait could be bounded.
    ClockFailed,
    AdminCommandFailed,
    InvalidTransfer,
}

pub type NvmeResult<T> = Result<T, NvmeError>;
