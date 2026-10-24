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

//! The rule that turns checks into a verdict. GO only when checks ran, every
//! one of them passed, and the kind of image is known; anything else is
//! NO-GO, naming the first check that did not pass. A check that could not
//! run counts exactly as one that failed. A GO whose checks carry limits
//! says how many in its line.

use super::Overall;
use crate::check::{Check, Status};
use crate::ctx::Kind;

pub const DEV_GO: &str = "GO (development image, not a release)";

pub fn decide(kind: Kind, checks: &[Check]) -> (Overall, String) {
    if checks.is_empty() {
        return (Overall::NoGo, "NO-GO: no check ran".into());
    }
    let missed: Vec<&Check> = checks.iter().filter(|c| c.status != Status::Pass).collect();
    match (missed.first(), kind) {
        (Some(c), _) => {
            let (n, of) = (missed.len(), checks.len());
            (Overall::NoGo, format!("NO-GO: {n} of {of} checks did not pass; first {}", c.id))
        }
        (None, Kind::Release) => (Overall::Go, limited("GO", checks)),
        (None, Kind::Development) => (Overall::Go, limited(DEV_GO, checks)),
        (None, Kind::Unknown) => {
            (Overall::NoGo, "NO-GO: the kind of image was not established".into())
        }
    }
}

/// Every limit the passing checks carry, counted in the GO line itself so a
/// limited GO can never be read as a plain one.
fn limited(go: &str, checks: &[Check]) -> String {
    match checks.iter().map(|c| c.limits.len()).sum::<usize>() {
        0 => go.into(),
        1 => format!("{go}, 1 declared limit"),
        n => format!("{go}, {n} declared limits"),
    }
}
