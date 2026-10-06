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

//! The replies the wallet reads, built from plain values: what a follow saw,
//! each kept spend, a history entry, and a finished job's envelope. Pure, so
//! the wire harness (`shield_wire_proofs`) builds the very replies the
//! service sends and hands them to the wallet's own readers.

use shield_wire::Values;

/// One look at a spend, as the follow answers it.
pub struct Followed<'a> {
    pub state: &'a str,
    pub minutes: u32,
    pub times: u32,
    pub self_settle: bool,
    pub tx: Option<&'a str>,
    pub link: Option<&'a str>,
    pub refusal: Option<&'a str>,
}

/// The spend proved last.
pub fn follow(v: &mut Values, f: &Followed<'_>) {
    v.put("state", f.state)
        .put("minutes", &f.minutes.to_string())
        .put("times", &f.times.to_string())
        .put("self_settle", if f.self_settle { "1" } else { "0" });
    if let Some(tx) = f.tx {
        v.put("tx", tx);
    }
    if let Some(link) = f.link {
        v.put("link", link);
    }
    if let Some(why) = f.refusal {
        v.put("lander_refusal", why);
    }
}

/// A spend kept before the last, as one `earlier` value: its number, what
/// became of it, minutes since first published, whether its owner may settle
/// it, and its settlement, with `|` between, since a state is words.
pub fn earlier(v: &mut Values, id: &str, f: &Followed<'_>) {
    let line = format!(
        "{id}|{}|{}|{}|{}",
        f.state,
        f.minutes,
        if f.self_settle { "1" } else { "0" },
        f.tx.unwrap_or("")
    );
    v.put("earlier", &line);
}

/// One history entry, as one `entry` value of a state reply.
pub fn entry(kind: &str, coin: &str, amount: &str, stage: &str, tx: &str, at: u64) -> String {
    format!("{kind}|{coin}|{amount}|{stage}|{tx}|{at}")
}

/// A finished job's reply to RESULT: the envelope, then the job's own values,
/// any named as the envelope's going as `job_<name>`.
pub fn done(name: &str, job: &str) -> Values {
    let mut v = Values::new();
    v.put("job", name).put("state", "done").put_job(job);
    v
}
