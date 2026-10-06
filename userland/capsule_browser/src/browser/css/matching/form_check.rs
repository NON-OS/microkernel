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

use super::cx::Cx;
use super::form_group::{first_submit, form_owner, group_checked};
use super::form_kind::{input_kind, is_submit};
use super::form_select::selected;

/* :checked: a checkbox or radio with the checked attribute, or a selected
 * option. The markup is what the matcher sees; a script that changes
 * checkedness through the attribute changes it here too. */
pub(super) fn checked(cx: &Cx, id: usize) -> bool {
    let Some(n) = cx.element(id) else { return false };
    match n.tag.as_str() {
        "input" => matches!(input_kind(n), "checkbox" | "radio") && n.attr("checked").is_some(),
        "option" => selected(cx, id),
        _ => false,
    }
}

/* :indeterminate: a radio whose group has nothing checked, or a progress
 * without a value. A checkbox is indeterminate only through a script
 * property no markup carries, so none is here. */
pub(super) fn indeterminate(cx: &Cx, id: usize) -> bool {
    let Some(n) = cx.element(id) else { return false };
    match n.tag.as_str() {
        "input" => input_kind(n) == "radio" && !group_checked(cx, id),
        "progress" => n.attr("value").is_none(),
        _ => false,
    }
}

/* :default: a checkbox or radio checked by default, an option selected by
 * default, and the default button of a form: its first submit button in
 * tree order. */
pub(super) fn is_default(cx: &Cx, id: usize) -> bool {
    let Some(n) = cx.element(id) else { return false };
    match n.tag.as_str() {
        "input" if matches!(input_kind(n), "checkbox" | "radio") => n.attr("checked").is_some(),
        "option" => n.attr("selected").is_some(),
        _ if is_submit(n) => {
            form_owner(cx.dom, id).is_some_and(|f| first_submit(cx, f) == Some(id))
        }
        _ => false,
    }
}
