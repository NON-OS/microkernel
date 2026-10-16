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

//! One step of the exchange: the stage it is at, moved on once, and how
//! the request ended. The handshake has one end from the hello to the
//! checked flight; past it the exchange fails, nothing of the request out.

use alloc::vec::Vec;

use super::super::super::handshake_end::over;
use super::super::super::WalletRoute;
use super::said::SLOW;
use super::state::Exchange;
use super::types::{Ended, Next, Stage};

impl Exchange {
    /// One step; Some once the request has ended.
    pub fn step(&mut self) -> Option<Ended> {
        let ended = match self.advance() {
            Ok(None) => return None,
            Ok(Some(plain)) => return Some(Ended::Answer(plain)),
            Err(why) if self.request_out() => Ended::Unknown(why),
            Err(why) => Ended::Failed(why),
        };
        /* A host that did not answer, refused, or did not prove itself is
         * left for the next on its network. A route that is down says
         * nothing of the host. */
        if !matches!(self.route, WalletRoute::Down(_)) {
            crate::wallet::chain::rpc_failed(self.host);
        }
        Some(ended)
    }

    /// Whether any of the request may have left: from the first write of
    /// it on. Before that only the handshake went, which asks nothing.
    fn request_out(&self) -> bool {
        matches!(self.stage, Stage::Send { next: Next::Answer } | Stage::Answer)
    }

    fn advance(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        let shaking =
            matches!(self.stage, Stage::Send { next: Next::Flight } | Stage::Flight | Stage::Check);
        if shaking && over(self.handshake_end, nonos_libc::mk_uptime_ms().max(0) as u64) {
            return Err(SLOW);
        }
        match &self.stage {
            Stage::Open(_) => self.open(),
            Stage::Hello => self.hello(),
            Stage::Send { next } => {
                let next = *next;
                self.send(next)
            }
            Stage::Flight => self.flight(),
            Stage::Check => self.check().map(|()| None),
            Stage::Answer => self.answer(),
        }
    }
}
