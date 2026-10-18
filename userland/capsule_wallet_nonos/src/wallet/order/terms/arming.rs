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
 * When Buy may act on the consent (T1's wallet side, before the trusted
 * UI path): only once the consent is drawn whole and the wallet's window
 * holds focus, and then not for ARM_MS, which starts again whenever focus
 * is lost and won back. Focus lost while the consent shows sends the
 * purchase back to reading its terms. Times are mk_uptime_ms, monotonic.
 */

/// How long the wallet must hold focus over the drawn consent first.
pub const ARM_MS: u64 = 1_000;

/// Whether Buy acts at `now`: the consent `drawn`, the wallet `focused`,
/// and focused unbroken since `since`, at least ARM_MS ago.
pub fn armed(drawn: bool, focused: bool, since: u64, now: u64) -> bool {
    drawn && focused && now.checked_sub(since).is_some_and(|held| held >= ARM_MS)
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Guard {
    /// The consent has been drawn whole on the wallet's window.
    pub drawn: bool,
    /// Since when the wallet has held focus unbroken over the drawn consent.
    pub since: Option<u64>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Look {
    Held(Guard),
    /// Focus held over the drawn consent was lost: read the terms again.
    Lost,
}

/// One look at focus at `now`, `focused` as the window manager said.
pub fn look(g: Guard, focused: bool, now: u64) -> Look {
    match (g.drawn, focused, g.since) {
        (false, _, _) => Look::Held(Guard::default()),
        (true, true, None) => Look::Held(Guard { since: Some(now), ..g }),
        (true, false, Some(_)) => Look::Lost,
        (true, _, _) => Look::Held(g),
    }
}

/// Whether Buy acts on `g` at `now`, after one more look at focus.
pub fn buy_armed(g: Guard, focused: bool, now: u64) -> bool {
    match look(g, focused, now) {
        Look::Held(g) => g.since.is_some_and(|since| armed(g.drawn, focused, since, now)),
        Look::Lost => false,
    }
}
