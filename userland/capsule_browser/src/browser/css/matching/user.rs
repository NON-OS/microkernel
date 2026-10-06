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

use crate::browser::css::selector::UserState;
use crate::browser::dom::Dom;

use super::state::element_state;

/* Parent hops an ancestor test climbs before giving up; a script can build
 * a parent chain that loops. */
const MAX_HOPS: u32 = 512;

/* The interaction pseudo-classes from the element state: :hover and
 * :active hold on the hovered or pressed element and every ancestor of it,
 * :focus-within on the focused element and its ancestors, :focus and
 * :target on the one element, :focus-visible on a keyboard focus. */
pub(super) fn user_matches(dom: &Dom, id: usize, s: UserState) -> bool {
    let st = element_state();
    match s {
        UserState::Hover => st.hovered.is_some_and(|h| holds(dom, id, h)),
        UserState::Active => st.active.is_some_and(|a| holds(dom, id, a)),
        UserState::Focus => st.focused == Some(id),
        UserState::FocusVisible => st.focus_visible && st.focused == Some(id),
        UserState::FocusWithin => st.focused.is_some_and(|f| holds(dom, id, f)),
        UserState::Target => st.target == Some(id),
    }
}

/* `anc` is `node` or one of its ancestors. */
fn holds(dom: &Dom, anc: usize, mut node: usize) -> bool {
    for _ in 0..MAX_HOPS {
        if node == anc {
            return true;
        }
        match dom.nodes.get(node) {
            Some(n) if n.parent != node && node != 0 => node = n.parent,
            _ => return false,
        }
    }
    false
}
