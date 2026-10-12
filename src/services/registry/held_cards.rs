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

//! Which stack each network card is held to (held.rs says why).

use super::super::nics::{Link, NICS};

/// The stack a network card is held to, when `endpoint` is one. The cards
/// come from nics.rs, so a card the stack binds is never left unheld.
pub(super) fn card_callers(endpoint: &str) -> Option<&'static [&'static str]> {
    let (_, link) = NICS.iter().find(|(card, _)| *card == endpoint)?;
    Some(match link {
        Link::Wired => WIRED_STACK,
        Link::Wifi => WIFI_STACK,
    })
}

/// net.core carries the frames and asks each card for its link; net.l2 finds
/// the wired cards.
const WIRED_STACK: &[&str] = &["net.core", "net.l2"];
/// A Wi-Fi card answers net.core, which carries its frames, and the panels
/// that scan and join: Settings, in its first window or a later one
/// (`src/userspace/capsule_settings/spawn.rs`), and the setup wizard. net.l2
/// binds only a wired card.
const WIFI_STACK: &[&str] =
    &["net.core", "app.settings", "app.settings.1", "app.settings.2", "app.setup_wizard"];
