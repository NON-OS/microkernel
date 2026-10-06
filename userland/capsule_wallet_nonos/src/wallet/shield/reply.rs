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

/*
 * Reading the shield service's replies: a follow's look at the last spend
 * and at each kept one, and the history a state reply carries. Pure, so the
 * wire harness (`shield_wire_proofs`) reads the very replies the service's
 * own builders make.
 */

use alloc::string::String;
use alloc::vec::Vec;

use shield_wire::{field, fields};

use crate::wallet::state::shield_log::{Entry, Kind, Stage};

/* A spend its owner settled from their own account, followed until it lands. */
pub const SETTLING: &str = "settling from your account";

/* The last spend, as a follow job's result reads. Its state is the job's
 * own, renamed by the service under its envelope's `state`. */
pub struct Last<'a> {
    pub said: &'a str,
    pub minutes: Option<u32>,
    pub self_settle: bool,
    pub published: bool,
    pub refusal: Option<&'a str>,
    pub tx: &'a str,
}

/* None when the reply does not say where the spend is: nothing is taken
 * from it, rather than reading it as a spend still waiting. */
pub fn last(body: &str) -> Option<Last<'_>> {
    Some(Last {
        said: field(body, "job_state")?,
        minutes: field(body, "minutes").and_then(|m| m.parse().ok()),
        self_settle: field(body, "self_settle") == Some("1"),
        published: field(body, "published") == Some("1"),
        refusal: field(body, "lander_refusal"),
        tx: field(body, "tx").unwrap_or(""),
    })
}

/* Whether a proved spend's publication reached a lander that took it: a
 * publication record kept while the lander refused is not one. */
pub fn taken(body: &str) -> bool {
    field(body, "published") == Some("1") && field(body, "lander_refusal").is_none()
}

/* What a landing says, with its transaction when the service named one. */
pub fn landed_news(what: &str, tx: &str) -> String {
    if tx.is_empty() {
        alloc::format!("{what} landed.")
    } else {
        alloc::format!("{what} landed ({tx}).")
    }
}

/* Whether a follow's state is the spend's end: in a block, its own or
 * another's. */
pub fn landed(said: &str) -> bool {
    matches!(said, "landed" | "spent elsewhere")
}

/* What a follow's state says, for a spend its owner was or was not settling. */
pub fn said(said: &str, settling: bool) -> &'static str {
    match said {
        "landed" => "landed",
        "spent elsewhere" if settling => "landed from your account",
        "spent elsewhere" => "landed by another route",
        _ if settling => SETTLING,
        "republished" => "handed to a lander again",
        "unpublished" => "kept, no lander took it",
        _ => "waiting for a lander",
    }
}

/* One kept spend, as one `earlier` value. */
pub struct Kept<'a> {
    pub id: &'a str,
    pub state: &'a str,
    pub minutes: u32,
    pub self_settle: bool,
    pub tx: &'a str,
}

fn kept(line: &str) -> Option<Kept<'_>> {
    let mut p = line.split('|');
    let id = p.next()?;
    let state = p.next()?;
    let minutes = p.next()?.parse().ok()?;
    let self_settle = p.next()? == "1";
    let tx = p.next()?;
    Some(Kept { id, state, minutes, self_settle, tx })
}

/* Every kept spend the follow saw, oldest first; None when the service
 * could not read them this time, so nothing is let go on a failed read. */
pub fn kept_all(body: &str) -> Option<Vec<Kept<'_>>> {
    if field(body, "earlier_why").is_some() {
        return None;
    }
    Some(fields(body, "earlier").filter_map(kept).collect())
}

/* The history a state reply carries, oldest first; None when it carries
 * none, as when the store would not give it (`history_why`): the list
 * shown before stays. */
pub fn entries(body: &str) -> Option<Vec<Entry>> {
    /* An entry the wallet cannot read is skipped, never shown as another. */
    (field(body, "history") == Some("1")).then(|| fields(body, "entry").filter_map(entry).collect())
}

fn entry(line: &str) -> Option<Entry> {
    let mut p = line.split('|');
    let kind = match p.next()? {
        "deposit" => Kind::Deposit,
        "sent" => Kind::Send,
        "withdrawn" => Kind::Withdraw,
        "received" => Kind::Received,
        _ => return None,
    };
    /* A coin this wallet does not know is not listed as another. */
    let asset = match p.next()? {
        "ETH" => 0,
        "NOX" => 1,
        _ => return None,
    };
    let amount = String::from(p.next()?);
    let stage = match p.next()? {
        "on its way" => Stage::Sent,
        "settling" => Stage::Settling,
        "done" => Stage::Settled,
        "taken back" => Stage::Failed,
        _ => return None,
    };
    let tx = p.next().filter(|t| !t.is_empty()).map(String::from);
    Some(Entry { kind, asset, amount, stage, tx })
}
