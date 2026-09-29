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

//! A flight handed over whole, for the calls that take one buffer.

use super::types::{HandshakeState, Progress, Start};
use crate::flight::ClientFlight;

impl HandshakeState {
    /// Key the handshake and take every record of `flight` in one go. `None`
    /// if the flight does not start with a ServerHello that can be keyed.
    pub(crate) fn whole(
        client: &ClientFlight,
        flight: &[u8],
    ) -> Option<(HandshakeState, Progress)> {
        match HandshakeState::begin(client, flight) {
            Start::Ready(mut state) => {
                let progress = state.advance(flight);
                Some((state, progress))
            }
            Start::Waiting | Start::Alert(_) | Start::Retry | Start::Unusable => None,
        }
    }
}
