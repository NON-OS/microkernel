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

//! A sealed frame, handed to the stage that may receive it.

use super::stage::Stage;
use super::Server;
use crate::{Attestation, HostSigner, Rand, Roles, Why};
use nsh_wire::{Kind, WireError};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    pub(super) fn sealed(
        &mut self,
        kind: Kind,
        body: &[u8],
        now_ms: u64,
        rand: Rand<'_>,
    ) -> Result<(), Why> {
        match self.stage {
            Stage::UserAuth => self.user_auth(kind, body, now_ms),
            Stage::Admitting => match kind {
                Kind::Abort => self.peer_abort(body),
                _ => Err(Why::Wire(WireError::Order)),
            },
            Stage::Session => self.session(kind, body, now_ms, rand),
            Stage::ClientHello | Stage::Over => Err(Why::Wire(WireError::Order)),
        }
    }
}
