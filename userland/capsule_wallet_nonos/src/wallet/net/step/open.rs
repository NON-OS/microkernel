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

//! A connection to the RPC host opened a step at a time, through route link
//! and only over Nym or Anyone (`private_route`): each step asks the proxy
//! once, for a slice. The wallet never opens a socket of its own.

use nonos_route_link::{RouteOpening, Slice};

use super::super::link::Link;
use super::super::private_route::{refused, WAITING};
use super::super::WalletRoute;
use super::SLICE_MS;

const NO_ANSWER: &str = "the RPC host did not answer";

pub enum Opening {
    Routed {
        opening: RouteOpening,
        patience_ms: u64,
    },
    /// No route the wallet may take, said on the first step.
    Down(&'static str),
}

impl Opening {
    pub fn begin(route: WalletRoute, host: &'static str) -> Opening {
        if let Some(why) = refused(route) {
            return Opening::Down(why);
        }
        let Some(link) = route.link() else { return Opening::Down(WAITING) };
        match RouteOpening::begin(link, host, 443, SLICE_MS) {
            Ok(opening) => Opening::Routed { opening, patience_ms: route.patience_ms() },
            Err(why) => Opening::Down(why),
        }
    }

    /// One step: the link once open, None while it is opening, or why it
    /// failed.
    pub fn step(&mut self) -> Result<Option<Link>, &'static str> {
        match self {
            Opening::Down(why) => Err(why),
            Opening::Routed { opening, patience_ms } => match opening.step(SLICE_MS) {
                Slice::Waiting => Ok(None),
                Slice::Failed(why) => Err(why),
                Slice::Done => {
                    let patience_ms = *patience_ms;
                    let done = core::mem::replace(self, Opening::Down(NO_ANSWER));
                    let Opening::Routed { opening, .. } = done else { return Err(NO_ANSWER) };
                    Ok(Some(Link { stream: opening.into_stream(), patience_ms }))
                }
            },
        }
    }
}
