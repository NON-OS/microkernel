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

//! Time, which only the host can read. Before `AUTH_OK` the handshake is
//! held to its deadline. While the session runs, the shell is stopped at
//! the grant's end, the role's longest session or the idle timeout, and
//! at once if the grant is revoked; a running escalation ends at its own
//! end or when its grant is revoked.

use super::stage::Stage;
use super::Server;
use crate::{Attestation, HostSigner, Rand, Roles, Why};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    /// Call at least once a second with the host clock.
    pub fn tick(&mut self, now_ms: u64, rand: Rand<'_>) {
        match self.stage {
            Stage::ClientHello | Stage::UserAuth | Stage::Admitting => {
                if now_ms.saturating_sub(self.started_ms) >= self.cfg.handshake_ms {
                    self.refuse(Why::HandshakeTime, now_ms, rand);
                }
            }
            Stage::Session => self.session_tick(now_ms, rand),
            Stage::Over => {}
        }
    }
}
