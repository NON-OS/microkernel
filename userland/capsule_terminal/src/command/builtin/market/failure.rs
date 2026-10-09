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

//! Why a market call brought nothing back to read, said once, plainly.

use alloc::format;

use nonos_market_proto::status::E_NODATA;
use nonos_market_proto::{status_name, ReplyError};

use crate::command::output::Output;

/*
 * What ENODATA means when one listing was asked for. The market answers it
 * both for an id it does not list and when it holds no catalogue at all,
 * so neither is claimed; `market list` tells them apart.
 */
pub(super) const NO_LISTING: &str =
    "no listing has that id, or this machine has no signed catalogue; `market list` says which";

pub(super) enum Failure {
    /// No `market.index` in the service table.
    NotAnnounced,
    /// The call itself failed or timed out; its errno.
    Call(i64),
    /// A reply that is not believed, or a refusal.
    Reply(ReplyError),
    /// A believed reply whose body does not parse.
    Malformed,
}

impl Failure {
    /// One line. `nodata` is what ENODATA means for this question: no
    /// catalogue, or no such listing.
    pub(super) fn say(self, out: &mut Output<'_>, nodata: &str) {
        let line = match self {
            Failure::NotAnnounced => {
                "market: the market service (market.index) has not announced itself".into()
            }
            Failure::Call(rc) => format!("market: the market did not answer (errno {})", -rc),
            Failure::Reply(ReplyError::Short) => "market: the market's reply was too short".into(),
            Failure::Reply(ReplyError::Foreign) => {
                "market: a reply came back that is not the market's; nothing was read".into()
            }
            Failure::Reply(ReplyError::Stale) => {
                "market: the reply answered an earlier request; nothing was read, ask again".into()
            }
            Failure::Reply(ReplyError::Truncated) => {
                "market: the market's reply was cut short".into()
            }
            Failure::Reply(ReplyError::Status(E_NODATA)) => format!("market: {nodata} (ENODATA)"),
            Failure::Reply(ReplyError::Status(s)) => match status_name(s) {
                Some(name) => format!("market: refused: {name} ({s})"),
                None => format!("market: refused with status {s}"),
            },
            Failure::Malformed => {
                "market: the market sent a reply this Terminal cannot read".into()
            }
        };
        out.writeln(line.as_bytes());
    }
}
