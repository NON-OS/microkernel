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

//! An eject in flight: waiting on a holder, or on `_STA` after `_EJ0`.

use nonos_aml::namespace::Id;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// A function below is still held; asked again on the tick.
    Held,
    /// `_EJ0` ran; waiting for `_STA` to say the device is gone.
    Verify { until_ms: u64 },
}

pub struct Pending {
    pub id: Id,
    pub stage: Stage,
    pub next_ms: u64,
}
