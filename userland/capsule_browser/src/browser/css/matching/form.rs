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

use crate::browser::css::selector::FormState;

use super::cx::Cx;
use super::form_check::{checked, indeterminate, is_default};
use super::form_disabled::disabled;
use super::form_kind::is_control;
use super::form_range::range_state;
use super::form_required::{optional, required};
use super::form_text::{placeholder_shown, read_write};
use super::form_valid::validity;

/* The form pseudo-classes, from markup: attributes as written, and the
 * element's place in its form, fieldset, select or radio group. */
pub(super) fn form_matches(cx: &Cx, id: usize, f: FormState) -> bool {
    let Some(node) = cx.element(id) else {
        return false;
    };
    match f {
        FormState::Enabled => is_control(&node.tag) && !disabled(cx.dom, id),
        FormState::Disabled => is_control(&node.tag) && disabled(cx.dom, id),
        FormState::Checked => checked(cx, id),
        FormState::Default => is_default(cx, id),
        FormState::Indeterminate => indeterminate(cx, id),
        FormState::Required => required(node),
        FormState::Optional => optional(node),
        FormState::ReadWrite => read_write(cx.dom, id),
        FormState::ReadOnly => !read_write(cx.dom, id),
        FormState::PlaceholderShown => placeholder_shown(cx.dom, id),
        FormState::Valid => validity(cx, id) == Some(true),
        FormState::Invalid => validity(cx, id) == Some(false),
        FormState::InRange => range_state(node) == Some(true),
        FormState::OutOfRange => range_state(node) == Some(false),
    }
}
