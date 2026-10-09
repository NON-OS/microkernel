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

use crate::browser::css::selector::Pseudo as P;

use super::pseudo_states::state;

/* Pseudo-classes Chromium accepts whose condition cannot arise in a NONOS
 * document: no visited history, shadow tree, fullscreen, autofill, popover,
 * modal, media or view-transition state, user edit or scrollbar part. */
const NEVER: [&str; 30] = [
    "visited",
    "autofill",
    "-webkit-autofill",
    "fullscreen",
    "-webkit-full-screen",
    "-webkit-full-screen-ancestor",
    "-webkit-full-page-media",
    "-webkit-drag",
    "modal",
    "popover-open",
    "picture-in-picture",
    "user-invalid",
    "user-valid",
    "past",
    "future",
    "target-current",
    "xr-overlay",
    "active-view-transition",
    "window-inactive",
    "host",
    "horizontal",
    "vertical",
    "decrement",
    "increment",
    "start",
    "end",
    "double-button",
    "single-button",
    "no-button",
    "corner-present",
];

/* A pseudo-class without an argument, by lower-cased name. None for a name
 * Chromium does not accept either, which invalidates the selector. */
pub(super) fn plain(name: &str) -> Option<P> {
    Some(match name {
        "first-child" => P::FirstChild,
        "last-child" => P::LastChild,
        "only-child" => P::OnlyChild,
        "first-of-type" => P::FirstOfType,
        "last-of-type" => P::LastOfType,
        "only-of-type" => P::OnlyOfType,
        "empty" => P::Empty,
        "root" => P::Root,
        "scope" => P::Scope,
        "link" | "any-link" | "-webkit-any-link" => P::AnyLink,
        "defined" => P::Defined,
        "open" => P::Open,
        _ => return state(name).or_else(|| NEVER.contains(&name).then_some(P::Never)),
    })
}
