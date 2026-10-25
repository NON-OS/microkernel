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

use shield_wire::field;

#[path = "reply_entries.rs"]
mod reply_entries;
#[path = "reply_kept.rs"]
mod reply_kept;
#[path = "reply_said.rs"]
mod reply_said;
pub use reply_entries::entries;
pub use reply_kept::kept_all;
pub use reply_said::{landed, landed_news, said};

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
