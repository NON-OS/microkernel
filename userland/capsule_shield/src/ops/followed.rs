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

//! A shield_core follow, as the reply builders take it.

/// One `earlier` value per spend kept before the last: its number, what became of it, minutes
/// since it was first published, whether its owner may settle it, and its settlement, with `|`
/// between them, since a state is words.
/// A shield_core follow as the reply builders take it.
macro_rules! followed {
    ($s:expr) => {
        crate::reply::Followed {
            state: &$s.state,
            minutes: $s.minutes_since_first,
            times: $s.times_published,
            self_settle: $s.self_settle_offered,
            tx: $s.tx.as_deref(),
            link: $s.link.as_deref(),
            refusal: $s.refusal.as_deref(),
        }
    };
}

pub(super) use followed;
