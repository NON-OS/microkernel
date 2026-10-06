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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BgaError {
    BrokerCallFailed(i64),
}

pub type BgaResult<T> = Result<T, BgaError>;

/// The words a failed bring-up attempt reports, carried into the single line
/// the shared schedule logs when the driver gives up. A machine with no
/// adapter never gets this far: discovery decides that before any attempt,
/// and the driver leaves with `EXIT_ABSENT`.
pub const fn reason(e: BgaError) -> &'static str {
    match e {
        BgaError::BrokerCallFailed(_) => "bga: broker refused a claim or grant",
    }
}
