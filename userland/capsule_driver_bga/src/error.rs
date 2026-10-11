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
    /// The DISPI id register holds no Bochs id: not an adapter to drive.
    NotDispi,
    /// Not even the driver's own mode fits in the adapter's VRAM.
    NoMode,
    /// The mode registers read back other than written.
    ModeRefused,
    /// The shadow the compositor draws into could not be made or shared.
    Surface(i64),
}

pub type BgaResult<T> = Result<T, BgaError>;

/// The words a failed bring-up attempt reports, carried into the single line
/// the shared schedule logs when the driver gives up. A machine with no
/// adapter never gets this far: discovery decides that before any attempt,
/// and the driver leaves with `EXIT_ABSENT`.
pub const fn reason(e: BgaError) -> &'static str {
    match e {
        BgaError::BrokerCallFailed(_) => "bga: broker refused a claim or grant",
        BgaError::NotDispi => "bga: no DISPI id in the register BAR",
        BgaError::NoMode => "bga: VRAM too small for any mode",
        BgaError::ModeRefused => "bga: the adapter did not take the mode",
        BgaError::Surface(_) => "bga: shadow surface refused",
    }
}
