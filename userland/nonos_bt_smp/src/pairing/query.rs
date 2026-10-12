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

//! What the caller and the other steps ask of the initiator.

use super::initiator::{Initiator, TIMEOUT_MS};
use super::state::State;
use crate::method::Method;

impl Initiator {
    pub fn state(&self) -> State {
        self.state
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub(super) fn expired(&self, now_ms: u64) -> bool {
        !self.state.finished() && now_ms.saturating_sub(self.last_ms) >= TIMEOUT_MS
    }

    pub(super) fn is_passkey(&self) -> bool {
        !matches!(self.method, Method::JustWorks | Method::NumericComparison)
    }
}
